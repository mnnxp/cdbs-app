mod catalog_spec;
mod search_bar;

pub use catalog_spec::CatalogSpec;
pub use search_bar::SearchBar;

use crate::services::compare_op_uuid;
use crate::types::UUID;
use crate::gqls::component::search_by_components::IptSearchArg;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct SearchArg {
    pub search: String,
    pub by_params: bool,
    pub by_specs: bool,
    pub by_keywords: bool,
    pub company_uuid: Option<UUID>,
    pub user_uuid: Option<UUID>,
    pub standard_uuid: Option<UUID>,
    pub service_uuid: Option<UUID>,
    pub favorite: bool,
    pub spec_id: Option<i64>,
}

impl SearchArg {
    /// If the root catalog (spec_id is 1) is passed, it is set to None
    pub(crate) fn by_spec_id(spec_id: usize) -> Self {
        Self {
            spec_id: Self::convert_spec_id(spec_id),
            ..Default::default()
        }
    }

    pub(crate) fn set_spec_id(&mut self, spec_id: usize) {
        self.spec_id = Self::convert_spec_id(spec_id);
    }

    pub(crate) fn convert_spec_id(spec_id: usize) -> Option<i64> {
        match spec_id > 1 {
            true => Some(spec_id as i64),
            false => None,
        }
    }

    /// Compares arguments without the search argument
    pub(crate) fn partial_comparison(&self, second: &SearchArg) -> bool {
        if self.by_params != second.by_params { return false }
        if self.by_specs != second.by_specs { return false }
        if self.by_keywords != second.by_keywords { return false }
        if !compare_op_uuid(&self.company_uuid, &second.company_uuid) {
            return false
        }
        if !compare_op_uuid(&self.user_uuid, &second.user_uuid) {
            return false
        }
        if !compare_op_uuid(&self.standard_uuid, &second.standard_uuid) {
            return false
        }
        if self.favorite != second.favorite { return false }
        if self.spec_id != second.spec_id { return false }
        true
    }
}

impl IptSearchArg {
    pub(crate) fn get_ipt(arg: &SearchArg) -> Self {
        IptSearchArg {
            search: arg.search.clone(),
            byParams: arg.by_params,
            bySpecs: arg.by_specs,
            byKeywords: arg.by_keywords,
            companyUuid: arg.company_uuid.clone(),
            userUuid: arg.user_uuid.clone(),
            standardUuid: arg.standard_uuid.clone(),
            serviceUuid: arg.service_uuid.clone(),
            favorite: arg.favorite,
            specId: arg.spec_id,
        }
    }
}