pub enum Resource {
    RDF(Container),
    NonRDF(RawResource)
}

pub enum ContainerType {
    Basic,
    Direct,
    Indirect
}

pub struct Container {
    container_type: ContainerType
}

impl Container {
    pub fn new(container_type: ContainerType) -> Self {
        Self { container_type }
    }
}

pub enum RawResourceType {
    Text,
    Image,
    Blob
}
pub struct RawResource {
    raw_resource_type: RawResourceType,
    data: Vec<u8>
}

impl RawResource {
    pub fn new(raw_resource_type: RawResourceType, data: Vec<u8>) -> Self {
        Self { raw_resource_type, data }
    }
}
