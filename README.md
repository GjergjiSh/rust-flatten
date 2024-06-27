Let's discuss the issue in more detail so you understand what I want.
I have a struct that i will eventually anotate with derive(CharacteristicContainer)

```rust
#[derive(CharacteristicContainer)]
struct Parent {
    uid: u32,
    child: Child,
}

#[derive(Clone, Copy, Debug)]
struct Child {
    uid: u32,
}
```

As you can see the parent struct contains a field of Type child which is also a struct.
I am working on a custom serializer for my structs that write sthe output to an A2L file.
In this case the parent and child members represent characteristics in the A2L file.

when i write the Parent to the A2L file i expected this to be written

/begin CHARACTERSITIC Parent.uid 0x00000000 u8 (other data irrelevant for not)
/begin CHARACTERSITIC Parent.Child.uid 0x00000000 u8 (other data irrelevant for not)

This is why i asked for the ability to flatten the struct so that the child fields are written as if they were part of the parent struct.

However this seems a bit too tricky
What i need is some way to unpack the nested structs in the parent struct so i get an intermediary representation 
The idea here is that A2L does not have a concept of strcuts but it expects only fields for characteristics

We need to figure out a way to implement this

# The intermediary representation



```

characteristic {
    name
    datatype
}

#[derive(ToA2l)]
struct Child {
    uid: u32,
}

#[derive(ToA2l)]
struct Parent {
    uid: u32,
    child: Child,
}

parent.to_a2l() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    characteristics.push(Characteristic {
        name: "Parent.uid",
        datatype: "u32",
    });
    characteristics.push(Characteristic {
        name: "Parent.Child.uid",
        datatype: "u32",
    });
    characteristics
}

```

# Rewite

I have a struct that i will eventually anotate with derive(CharacteristicContainer)

```rust
#[derive(CharacteristicContainer)]
struct Parent {
    uid: u32,
    child: Child,
}

#[derive(Clone, Copy, Debug)]
struct Child {
    uid: u32,
}
```

As you can see the parent struct contains a field of Type child which is also a struct.
I am working on a custom serializer for my structs that write sthe output to an A2L file.
In this case the parent and child members represent characteristics in the A2L file.

when i write the Parent to the A2L file i expected this to be written

/begin CHARACTERSITIC Parent.uid 0x00000000 u8 (other data irrelevant for not)
/begin CHARACTERSITIC Parent.Child.uid 0x00000000 u8 (other data irrelevant for not)

I need to figure out a way to implement this
basically i need a way to unpack the nested structs

characteristic {
    name
    datatype
}

#[derive(ToA2l)]
struct Child {
    uid: u32,
}

#[derive(ToA2l)]
struct Parent {
    uid: u32,
    child: Child,
}

parent.to_a2l() -> Vec<Characteristic> {
    let mut characteristics = vec![];
    characteristics.push(Characteristic {
        name: "Parent.uid",
        datatype: "u32",
    });
    characteristics.push(Characteristic {
        name: "Parent.Child.uid",
        datatype: "u32",
    });
    characteristics
}


# Workaround

if let Some(nested_characteristics) = <#field_type as CharacteristicContainer>::to_a2l_optional(&self.#field_name) {
    ...
}

This line attempts to call to_a2l_optional on every field of the struct, regardless of its type. The to_a2l_optional method is a part of the CharacteristicContainer trait, so this line effectively requires every field type (#field_type) within your struct to implement the CharacteristicContainer trait. Since primitive types like u32 do not implement this trait (and you cannot directly implement external traits on external types due to Rust's orphan rule), the compiler will complain.

To resolve this issue, you need a way to conditionally call to_a2l_optional only on types that implement CharacteristicContainer, without requiring it for types that do not (like u32). Unfortunately, Rust's type system does not directly support checking if a type implements a trait at runtime (trait object checks are compile-time).

One workaround is to use a procedural macro to generate different code based on the type of each field. However, this requires knowing the types at compile time and cannot be done with simple trait bounds or conditional checks in the trait implementation itself.

Another approach is to provide a default implementation of CharacteristicContainer for all types that do not need flattening, essentially making the operation a no-op for them. This can be somewhat achieved using a blanket implementation, but Rust does not allow blanket implementations that conflict with more specific implementations.

Given these constraints, the most straightforward solution might be to reconsider the design. If the goal is to only flatten certain complex types (and ignore or handle primitives differently), you might need to explicitly mark which fields or types should be flattened, or provide a mechanism to bypass the flattening process for types that do not support it.

For example, you could introduce a marker trait or use an attribute macro to specify which fields should be flattened, avoiding the attempt to flatten types that do not support it. This approach requires manual annotation but offers greater control over the flattening process.