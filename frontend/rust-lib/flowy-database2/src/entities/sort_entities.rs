use flowy_derive::{ProtoBuf, ProtoBuf_Enum};
use validator::Validate;

use crate::entities::FieldType;
use crate::services::sort::{Sort, SortCondition};

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct SortPB {
  #[pb(index = 1)]
  pub id: String,

  #[pb(index = 2)]
  pub field_id: String,

  #[pb(index = 3)]
  pub field_type: FieldType,

  #[pb(index = 4)]
  pub condition: SortConditionPB,
}

impl std::convert::From<&Sort> for SortPB {
  fn from(sort: &Sort) -> Self {
    Self {
      id: sort.id.clone(),
      field_id: sort.field_id.clone(),
      field_type: sort.field_type,
      condition: sort.condition.into(),
    }
  }
}

impl std::convert::From<Sort> for SortPB {
  fn from(sort: Sort) -> Self {
    Self {
      id: sort.id,
      field_id: sort.field_id,
      field_type: sort.field_type,
      condition: sort.condition.into(),
    }
  }
}

#[derive(Eq, PartialEq, ProtoBuf, Debug, Default, Clone)]
pub struct RepeatedSortPB {
  #[pb(index = 1)]
  pub items: Vec<SortPB>,
}

impl std::convert::From<Vec<Sort>> for RepeatedSortPB {
  fn from(revs: Vec<Sort>) -> Self {
    RepeatedSortPB {
      items: revs.into_iter().map(|sort| sort.into()).collect(),
    }
  }
}

impl std::convert::From<Vec<SortPB>> for RepeatedSortPB {
  fn from(items: Vec<SortPB>) -> Self {
    Self { items }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, ProtoBuf_Enum)]
#[repr(u8)]
pub enum SortConditionPB {
  Ascending = 0,
  Descending = 1,
}
impl std::default::Default for SortConditionPB {
  fn default() -> Self {
    Self::Ascending
  }
}

impl std::convert::From<SortCondition> for SortConditionPB {
  fn from(condition: SortCondition) -> Self {
    match condition {
      SortCondition::Ascending => SortConditionPB::Ascending,
      SortCondition::Descending => SortConditionPB::Descending,
    }
  }
}
impl std::convert::From<SortConditionPB> for SortCondition {
  fn from(condition: SortConditionPB) -> Self {
    match condition {
      SortConditionPB::Ascending => SortCondition::Ascending,
      SortConditionPB::Descending => SortCondition::Descending,
    }
  }
}

#[derive(ProtoBuf, Debug, Default, Clone, Validate)]
pub struct UpdateSortPayloadPB {
  #[pb(index = 1)]
  #[validate(custom = "lib_infra::validator_fn::required_not_empty_str")]
  pub view_id: String,

  #[pb(index = 2)]
  #[validate(custom = "lib_infra::validator_fn::required_not_empty_str")]
  pub field_id: String,

  #[pb(index = 3)]
  pub field_type: FieldType,

  /// Create a new sort if the sort_id is None
  #[pb(index = 4, one_of)]
  pub sort_id: Option<String>,

  #[pb(index = 5)]
  pub condition: SortConditionPB,
}

#[derive(ProtoBuf, Debug, Default, Clone, Validate)]
pub struct DeleteSortPayloadPB {
  #[pb(index = 1)]
  #[validate(custom = "lib_infra::validator_fn::required_not_empty_str")]
  pub view_id: String,

  #[pb(index = 2)]
  #[validate(custom = "lib_infra::validator_fn::required_not_empty_str")]
  pub sort_id: String,
}

#[derive(Debug, Default, ProtoBuf)]
pub struct SortChangesetNotificationPB {
  #[pb(index = 1)]
  pub view_id: String,

  #[pb(index = 2)]
  pub insert_sorts: Vec<SortPB>,

  #[pb(index = 3)]
  pub delete_sorts: Vec<SortPB>,

  #[pb(index = 4)]
  pub update_sorts: Vec<SortPB>,
}

impl SortChangesetNotificationPB {
  pub fn new(view_id: String) -> Self {
    Self {
      view_id,
      insert_sorts: vec![],
      delete_sorts: vec![],
      update_sorts: vec![],
    }
  }

  pub fn extend(&mut self, other: SortChangesetNotificationPB) {
    self.insert_sorts.extend(other.insert_sorts);
    self.delete_sorts.extend(other.delete_sorts);
    self.update_sorts.extend(other.update_sorts);
  }

  pub fn is_empty(&self) -> bool {
    self.insert_sorts.is_empty() && self.delete_sorts.is_empty() && self.update_sorts.is_empty()
  }
}

#[derive(Debug, Default, ProtoBuf)]
pub struct ReorderAllRowsPB {
  #[pb(index = 1)]
  pub row_orders: Vec<String>,
}

#[derive(Debug, Default, ProtoBuf)]
pub struct ReorderSingleRowPB {
  #[pb(index = 1)]
  pub row_id: String,

  #[pb(index = 2)]
  pub old_index: i32,

  #[pb(index = 3)]
  pub new_index: i32,
}
