use mongodb::{bson::{doc, oid::ObjectId}, Collection, Database};
use crate::models::shopping::{ShoppingList, Item};
use futures::stream::TryStreamExt;

pub struct ShoppingRepo {
    collection: Collection<ShoppingList>,
}

impl ShoppingRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("shopping_lists"),
        }
    }

    pub async fn create_list(&self, name: &str, user_id: &str) -> mongodb::error::Result<ObjectId> {
        let list = ShoppingList {
            id: None,
            name: name.to_string(),
            user_id: user_id.to_string(),
            items: vec![],
        };
        let insert_result = self.collection.insert_one(list, None).await?;
        Ok(insert_result.inserted_id.as_object_id().unwrap())
    }

    pub async fn update_list_name(&self, list_id: &str, name: &str) -> mongodb::error::Result<()> {
        let id = ObjectId::parse_str(list_id)?;
        self.collection.update_one(doc! {"_id": id}, doc! {"$set": {"name": name}}, None).await?;
        Ok(())
    }

    pub async fn delete_list(&self, list_id: &str) -> mongodb::error::Result<()> {
        let id = ObjectId::parse_str(list_id)?;
        self.collection.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }

    pub async fn add_item(&self, list_id: &str, item: Item) -> mongodb::error::Result<()> {
        let id = ObjectId::parse_str(list_id)?;
        self.collection.update_one(doc! {"_id": id}, doc! {"$push": {"items": item}}, None).await?;
        Ok(())
    }

    pub async fn update_item(&self, list_id: &str, item_id: &str, name: Option<String>, done: Option<bool>) -> mongodb::error::Result<()> {
        let id = ObjectId::parse_str(list_id)?;
        let mut update_doc = doc! {};

        if let Some(name) = name {
            update_doc.insert("items.$.name", name);
        }
        if let Some(done) = done {
            update_doc.insert("items.$.done", done);
        }

        self.collection.update_one(
            doc! {"_id": id, "items.id": item_id},
            doc! {"$set": update_doc},
            None,
        ).await?;
        Ok(())
    }

    pub async fn delete_item(&self, list_id: &str, item_id: &str) -> mongodb::error::Result<()> {
        let id = ObjectId::parse_str(list_id)?;
        self.collection.update_one(
            doc! {"_id": id},
            doc! {"$pull": {"items": {"id": item_id}}},
            None,
        ).await?;
        Ok(())
    }
}
