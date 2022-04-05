use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};
use netx_msgclient_lib::inventory;

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::overloads::DotNet;
    use interoptopus_backend_csharp::{Config, Generator};

    let config = Config {
        dll_name: "netx_msgclient_lib".to_string(),
        namespace_mappings: NamespaceMappings::new("rust_run"),
        ..Config::default()
    };

    Generator::new(config, inventory())
        .add_overload_writer(DotNet::new())
        //.add_overload_writer(Unity::new())
        .write_file("Interop.cs")?;

    Ok(())
}
