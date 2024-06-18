use async_graphql::*;

use repository::vaccine_course::vaccine_course_item::VaccineCourseItem;

#[derive(PartialEq, Debug)]
pub struct VaccineCourseItemNode {
    pub vaccine_course_item: VaccineCourseItem,
}

#[Object]
impl VaccineCourseItemNode {
    pub async fn id(&self) -> &str {
        &self.row().vaccine_course_item.id
    }

    pub async fn item_id(&self) -> &str {
        &self.row().item.id
    }

    pub async fn name(&self) -> &str {
        &self.row().item.name
    }
}

impl VaccineCourseItemNode {
    pub fn from_domain(vaccine_course_item: VaccineCourseItem) -> VaccineCourseItemNode {
        VaccineCourseItemNode {
            vaccine_course_item,
        }
    }

    pub fn row(&self) -> &VaccineCourseItem {
        &self.vaccine_course_item
    }
}
