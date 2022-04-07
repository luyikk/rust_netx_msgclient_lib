// Automatically generated by Interoptopus.

#ifndef example_hello_world
#define example_hello_world

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>




typedef struct messageclient messageclient;
typedef enum netxffierror
    {
    NETXFFIERROR_OK = 0,
    NETXFFIERROR_NULLPASSED = 1,
    NETXFFIERROR_PANIC = 2,
    NETXFFIERROR_ANYHOWERROR = 3,
    NETXFFIERROR_NOTCONNECT = 4,
    } netxffierror;

typedef struct user
    {
    uint8_t* nickname;
    int64_t session_id;
    } user;

typedef bool (*fptr_fn_u8_pconst_u8_rval_bool)(uint8_t x0, uint8_t* x1);

typedef void (*fptr_fn_pconst_u8_i64)(uint8_t* x0, int64_t x1);

typedef struct sliceuser
    {
    user* data;
    uint64_t len;
    } sliceuser;

typedef void (*fptr_fn_SliceUser)(sliceuser x0);


uint64_t my_api_guard();
netxffierror destroy(messageclient** context);
netxffierror new_by_config(messageclient** context, uint8_t* config);
netxffierror init(messageclient* context);
netxffierror connect_test(messageclient* context);
bool login(messageclient* context, uint8_t* nickname, fptr_fn_u8_pconst_u8_rval_bool callback);
netxffierror get_users(messageclient* context, fptr_fn_SliceUser callback);
netxffierror talk(messageclient* context, uint8_t* msg);
netxffierror to(messageclient* context, uint8_t* target, uint8_t* msg);
netxffierror ping(messageclient* context, uint8_t* target, int64_t time, fptr_fn_pconst_u8_i64 callback);

#ifdef __cplusplus
}
#endif

#endif /* example_hello_world */
