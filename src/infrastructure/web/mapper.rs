use crate::domain::domains::Domain;
use crate::infrastructure::web::dtos::{Dto, DtoPost};

impl From<Dto> for Domain {
    fn from(dto: Dto) -> Self {
        Domain {
            item_id: dto.item_id,
            name: dto.name,
            description: dto.description,
            price: dto.price,
            quantity: dto.quantity,
            stock: dto.stock,
            category: dto.category,
            url: dto.url,
            image_url: dto.image_url,
            is_active: dto.is_active,
        }
    }
}

impl From<Domain> for Dto {
    fn from(domain: Domain) -> Self {
        Dto {
            item_id: domain.item_id,
            name: domain.name,
            description: domain.description,
            price: domain.price,
            quantity: domain.quantity,
            stock: domain.stock,
            category: domain.category,
            url: domain.url,
            image_url: domain.image_url,
            is_active: domain.is_active,
        }
    }
}

impl From<DtoPost> for Domain {
    fn from(dto: DtoPost) -> Self {
        Domain {
            item_id: 0,
            name: dto.name,
            description: dto.description,
            price: dto.price,
            quantity: dto.quantity,
            stock: dto.stock,
            category: dto.category,
            url: dto.url,
            image_url: dto.image_url,
            is_active: dto.is_active,
        }
    }
}

impl From<Domain> for DtoPost {
    fn from(domain: Domain) -> Self {
        DtoPost {
            name: domain.name,
            description: domain.description,
            price: domain.price,
            quantity: domain.quantity,
            stock: domain.stock,
            category: domain.category,
            url: domain.url,
            image_url: domain.image_url,
            is_active: domain.is_active,
        }
    }
}
