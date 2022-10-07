# Inextensible Rust

There is no way to access private fields while extending an external struct due to the visibility and privacy of Rust.

Tried:

* [Extension traits](extension_trait/)
* [Newtype pattern](newtype_pattern/)

PS: An interesting method to break the rule using the `Deserialize` derive:

<https://twitter.com/CecileTonglet/status/1429020136535695363>

via <https://users.rust-lang.org/t/is-it-possible-to-access-a-structs-private-field-outside-the-module/63951/3>
