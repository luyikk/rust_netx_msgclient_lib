from __future__ import annotations
import ctypes
import typing

T = typing.TypeVar("T")
c_lib = None

def init_lib(path):
    """Initializes the native library. Must be called at least once before anything else."""
    global c_lib
    c_lib = ctypes.cdll.LoadLibrary(path)

    c_lib.my_api_guard.argtypes = []
    c_lib.destroy.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    c_lib.new_by_config.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.POINTER(ctypes.c_uint8)]
    c_lib.init.argtypes = [ctypes.c_void_p]
    c_lib.connect_test.argtypes = [ctypes.c_void_p]
    c_lib.login.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), callbacks.fn_u8_pconst_u8_rval_bool]
    c_lib.get_users.argtypes = [ctypes.c_void_p, callbacks.fn_SliceUser]
    c_lib.talk.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8)]
    c_lib.to.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.POINTER(ctypes.c_uint8)]
    c_lib.ping.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_uint8), ctypes.c_int64, callbacks.fn_pconst_u8_i64]

    c_lib.my_api_guard.restype = ctypes.c_uint64
    c_lib.destroy.restype = ctypes.c_int
    c_lib.new_by_config.restype = ctypes.c_int
    c_lib.init.restype = ctypes.c_int
    c_lib.connect_test.restype = ctypes.c_int
    c_lib.login.restype = ctypes.c_bool
    c_lib.get_users.restype = ctypes.c_int
    c_lib.talk.restype = ctypes.c_int
    c_lib.to.restype = ctypes.c_int
    c_lib.ping.restype = ctypes.c_int

    c_lib.destroy.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.new_by_config.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.init.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.connect_test.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.get_users.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.talk.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.to.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)
    c_lib.ping.errcheck = lambda rval, _fptr, _args: _errcheck(rval, 0)


def my_api_guard():
    return c_lib.my_api_guard()





TRUE = ctypes.c_uint8(1)
FALSE = ctypes.c_uint8(0)


def _errcheck(returned, success):
    """Checks for FFIErrors and converts them to an exception."""
    if returned == success: return
    else: raise Exception(f"Function returned error: {returned}")


class CallbackVars(object):
    """Helper to be used `lambda x: setattr(cv, "x", x)` when getting values from callbacks."""
    def __str__(self):
        rval = ""
        for var in  filter(lambda x: "__" not in x, dir(self)):
            rval += f"{var}: {getattr(self, var)}"
        return rval


class _Iter(object):
    """Helper for slice iterators."""
    def __init__(self, target):
        self.i = 0
        self.target = target

    def __iter__(self):
        self.i = 0
        return self

    def __next__(self):
        if self.i >= self.target.len:
            raise StopIteration()
        rval = self.target[self.i]
        self.i += 1
        return rval


class NetXFFIError:
    Ok = 0
    NullPassed = 1
    Panic = 2
    AnyHowError = 3
    NotConnect = 4


class User(ctypes.Structure):

    # These fields represent the underlying C data layout
    _fields_ = [
        ("nickname", ctypes.POINTER(ctypes.c_uint8)),
        ("session_id", ctypes.c_int64),
    ]

    def __init__(self, nickname: str = None, session_id: int = None):
        if nickname is not None:
            self.nickname = nickname
        if session_id is not None:
            self.session_id = session_id

    @property
    def nickname(self) -> str:
        return ctypes.Structure.__get__(self, "nickname")

    @nickname.setter
    def nickname(self, value: str):
        return ctypes.Structure.__set__(self, "nickname", value)

    @property
    def session_id(self) -> int:
        return ctypes.Structure.__get__(self, "session_id")

    @session_id.setter
    def session_id(self, value: int):
        return ctypes.Structure.__set__(self, "session_id", value)


class SliceUser(ctypes.Structure):
    # These fields represent the underlying C data layout
    _fields_ = [
        ("data", ctypes.POINTER(User)),
        ("len", ctypes.c_uint64),
    ]

    def __len__(self):
        return self.len

    def __getitem__(self, i) -> User:
        return self.data[i]

    def copied(self) -> SliceUser:
        """Returns a shallow, owned copy of the underlying slice.

        The returned object owns the immediate data, but not the targets of any contained
        pointers. In other words, if your struct contains any pointers the returned object
        may only be used as long as these pointers are valid. If the struct did not contain
        any pointers the returned object is valid indefinitely."""
        array = (User * len(self))()
        ctypes.memmove(array, self.data, len(self) * ctypes.sizeof(User))
        rval = SliceUser(data=ctypes.cast(array, ctypes.POINTER(User)), len=len(self))
        rval.owned = array  # Store array in returned slice to prevent memory deallocation
        return rval

    def __iter__(self) -> typing.Iterable[User]:
        return _Iter(self)

    def iter(self) -> typing.Iterable[User]:
        """Convenience method returning a value iterator."""
        return iter(self)

    def first(self) -> User:
        """Returns the first element of this slice."""
        return self[0]

    def last(self) -> User:
        """Returns the last element of this slice."""
        return self[len(self)-1]




class callbacks:
    """Helpers to define callbacks."""
    fn_SliceUser = ctypes.CFUNCTYPE(None, SliceUser)
    fn_u8_pconst_u8_rval_bool = ctypes.CFUNCTYPE(ctypes.c_bool, ctypes.c_uint8, ctypes.POINTER(ctypes.c_uint8))
    fn_pconst_u8_i64 = ctypes.CFUNCTYPE(None, ctypes.POINTER(ctypes.c_uint8), ctypes.c_int64)


class MessageClient:
    """ netx message lib"""
    __api_lock = object()

    def __init__(self, api_lock, ctx):
        assert(api_lock == MessageClient.__api_lock), "You must create this with a static constructor." 
        self._ctx = ctx

    @property
    def _as_parameter_(self):
        return self._ctx

    @staticmethod
    def new_by_config(config: str) -> MessageClient:
        """ new MessageClient obj
 config is json ServerOption"""
        ctx = ctypes.c_void_p()
        if not hasattr(config, "__ctypes_from_outparam__"):
            config = ctypes.cast(config, ctypes.POINTER(ctypes.c_uint8))
        c_lib.new_by_config(ctx, config)
        self = MessageClient(MessageClient.__api_lock, ctx)
        return self

    def __del__(self):
        c_lib.destroy(self._ctx, )
    def init(self, ):
        """ init"""
        return c_lib.init(self._ctx, )

    def connect_test(self, ):
        """ test connect"""
        return c_lib.connect_test(self._ctx, )

    def login(self, nickname: str, callback) -> bool:
        """ login
 callback args:
     success:bool
     msg:string
     ret:bool"""
        if not hasattr(nickname, "__ctypes_from_outparam__"):
            nickname = ctypes.cast(nickname, ctypes.POINTER(ctypes.c_uint8))
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_u8_pconst_u8_rval_bool(callback)

        return c_lib.login(self._ctx, nickname, callback)

    def get_users(self, callback):
        """ get all online users"""
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_SliceUser(callback)

        return c_lib.get_users(self._ctx, callback)

    def talk(self, msg: str):
        """ message to all online users"""
        if not hasattr(msg, "__ctypes_from_outparam__"):
            msg = ctypes.cast(msg, ctypes.POINTER(ctypes.c_uint8))
        return c_lib.talk(self._ctx, msg)

    def to(self, target: str, msg: str):
        """ message to target user"""
        if not hasattr(target, "__ctypes_from_outparam__"):
            target = ctypes.cast(target, ctypes.POINTER(ctypes.c_uint8))
        if not hasattr(msg, "__ctypes_from_outparam__"):
            msg = ctypes.cast(msg, ctypes.POINTER(ctypes.c_uint8))
        return c_lib.to(self._ctx, target, msg)

    def ping(self, target: str, time: int, callback):
        """ ping"""
        if not hasattr(target, "__ctypes_from_outparam__"):
            target = ctypes.cast(target, ctypes.POINTER(ctypes.c_uint8))
        if not hasattr(callback, "__ctypes_from_outparam__"):
            callback = callbacks.fn_pconst_u8_i64(callback)

        return c_lib.ping(self._ctx, target, time, callback)



