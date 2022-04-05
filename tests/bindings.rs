use csharp::my_inventory;
use interoptopus::util::NamespaceMappings;
use interoptopus::{Error, Interop};

#[test]
fn bindings_csharp() -> Result<(), Error> {
    use interoptopus_backend_csharp::overloads::DotNet;
    use interoptopus_backend_csharp::{Config, Generator};

    let config = Config {
        dll_name: "csharp".to_string(),
        namespace_mappings: NamespaceMappings::new("rust_run"),
        ..Config::default()
    };

    Generator::new(config, my_inventory())
        .add_overload_writer(DotNet::new())
        //.add_overload_writer(Unity::new())
        .write_file("Interop.cs")?;

    Ok(())
}
