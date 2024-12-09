use crate::domain::domains::Domain;
use crate::infrastructure::database::models::Model;

impl From<Model> for Domain {
    fn from(model: Model) -> Self {
        Domain {
            item_id: model.item_id,
            name: model.name,
            description: model.description,
            price: model.price,
            quantity: model.quantity,
            stock: model.stock,
            category: model.category,
            url: model.url,
            image_url: model.image_url,
            is_active: model.is_active,
        }
    }
}

impl From<Domain> for Model {
    fn from(domain: Domain) -> Self {
        Model {
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
