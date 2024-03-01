## Grpc_client_server 

#### Overview
This Rust project implements a client-server architecture using gRPC communication. It consists of client and server modules for handling CRUD operations.

#### Functionality
- `Client Module`: Facilitates calling functions and passing request parameters for CRUD operations to the server module.
- `Server Module`: Contains implementations for various CRUD operations based on the requests received from the client module.

#### Usage
Explore the functionalities provided by both the client and server modules to understand their purposes and usage.

#### Additional Information
- For Rust documentation, you can run `rustup doc` anywhere in the Rust environment. If using a Cargo project, you can generate and open documentation with `cargo doc --open`. To execute a specific binary file, you can use `cargo run -q --bin file_name`.


- If you want to make your server to be public in you wifi network make sure that your project  .exe file should be public by checkin here `Control Panel\System and Security\Windows Defender Firewall\Allowed apps`, here search your .exe app and make the checkbox write tick mark. then in you server address make sure to put the address same as your local wifi address ,
- for example local wifi address is ` 156.12.0.168`, then in your server file this should be the address.then your server should be accessible to anyone who is connected with the same wifi.