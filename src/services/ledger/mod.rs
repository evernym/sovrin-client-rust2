extern crate time;

pub mod merkletree;
pub mod types;
pub mod constants;

use self::types::{
    AttribOperation,
    GetAttribOperation,
    GetNymOperation,
    GetSchemaOperationData,
    GetSchemaOperation,
    NymOperation,
    NymOperationData,
    Request,
    SchemaOperation,
    SchemaOperationData,
    ClaimDefOperation,
    ClaimDefOperationData,
    GetClaimDefOperation,
    GetDdoOperation,
    NodeOperation,
    NodeOperationData
};
use errors::common::CommonError;
use utils::json::{JsonEncodable, JsonDecodable};

trait LedgerSerializer {
    fn serialize(&self) -> String;
}

pub struct LedgerService {}

impl LedgerService {
    pub fn new() -> LedgerService {
        LedgerService {}
    }

    pub fn build_nym_request(&self, identifier: &str, dest: &str, verkey: Option<&str>, _ref: Option<&str>,
                             data: Option<&str>, role: Option<&str>) -> Result<String, CommonError> {

        //TODO: check identifier, dest, verkey
        let req_id = LedgerService::get_req_id();

        let data = match data {
            Some(d) => Some(NymOperationData::from_json(d)
                .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?),
            _ => None
        };

        let operation = NymOperation::new(dest.to_string(),
                                          verkey.as_ref().map(|s| s.to_string()),
                                          _ref.as_ref().map(|s| s.to_string()),
                                          data,
                                          role.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid nym request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_nym_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetNymOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_nym request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_ddo_request(&self, identifier: &str, dest: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetDdoOperation::new(dest.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_ddo request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_attrib_request(&self, identifier: &str, dest: &str, hash: Option<&str>,
                                raw: Option<&str>, enc: Option<&str>) -> Result<String, CommonError> {
        if raw.is_none() && hash.is_none() && enc.is_none() {
            return Err(CommonError::InvalidStructure(format!("Either raw or hash or enc must be specified")))
        }
        let req_id = LedgerService::get_req_id();
        let operation = AttribOperation::new(dest.to_string(),
                                             hash.as_ref().map(|s| s.to_string()),
                                             raw.as_ref().map(|s| s.to_string()),
                                             enc.as_ref().map(|s| s.to_string()));
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid attrib request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_attrib_request(&self, identifier: &str, dest: &str, raw: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetAttribOperation::new(dest.to_string(),
                                                raw.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_attrib request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let data = SchemaOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = SchemaOperation::new(data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid schema request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_schema_request(&self, identifier: &str, data: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let data = GetSchemaOperationData::from_json(data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = GetSchemaOperation::new(data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_schema request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_claim_def_request(&self, identifier: &str, _ref: &str, signature_type: &str, data: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let data = ClaimDefOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = ClaimDefOperation::new(_ref.to_string(), signature_type.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid claim_def request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_get_claim_def_request(&self, identifier: &str, _ref: &str, signature_type: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let operation = GetClaimDefOperation::new(_ref.to_string(),
                                                  signature_type.to_string());
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid get_claim_def request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    pub fn build_node_request(&self, identifier: &str, dest: &str, data: &str) -> Result<String, CommonError> {
        let req_id = LedgerService::get_req_id();
        let data = NodeOperationData::from_json(&data)
            .map_err(|err| CommonError::InvalidStructure(format!("Invalid data json: {}", err.to_string())))?;
        let operation = NodeOperation::new(dest.to_string(), data);
        let request = Request::new(req_id,
                                   identifier.to_string(),
                                   operation);
        let request_json = Request::to_json(&request)
            .map_err(|err| CommonError::InvalidState(format!("Invalid node request json: {}", err.to_string())))?;
        Ok(request_json)
    }

    fn get_req_id() -> u64 {
        time::get_time().sec as u64 * (1e9 as u64) + time::get_time().nsec as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_nym_request_works_for_only_required_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"1","dest":"some_dest"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, None, None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_optional_fields() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let verkey = "some_verkey";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"1","dest":"some_dest","verkey":"some_verkey"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, Some(verkey), None, None, None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_data_json() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let data = r#"{"alias":"some_alias"}"#;

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"1","dest":"some_dest","data":{"alias":"some_alias"}}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, Some(data), None);
        assert!(nym_request.is_ok());
        let nym_request = nym_request.unwrap();
        assert!(nym_request.contains(expected_result));
    }

    #[test]
    fn build_nym_request_works_for_wrong_data_json() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let data = r#"{"field": "field"}"#;

        let nym_request = ledger_service.build_nym_request(identifier, dest, None, None, Some(data), None);
        assert!(nym_request.is_err());
    }

    #[test]
    fn build_get_nym_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"105","dest":"some_dest"}"#;

        let get_nym_request = ledger_service.build_get_nym_request(identifier, dest);
        assert!(get_nym_request.is_ok());
        let get_nym_request = get_nym_request.unwrap();
        assert!(get_nym_request.contains(expected_result));
    }

    #[test]
    fn build_get_ddo_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"120","dest":"some_dest"}"#;

        let get_ddo_request = ledger_service.build_get_ddo_request(identifier, dest);
        assert!(get_ddo_request.is_ok());
        let get_ddo_request = get_ddo_request.unwrap();
        assert!(get_ddo_request.contains(expected_result));
    }

    #[test]
    fn build_attrib_request_works_for_miss_attrib_field() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, None, None, None);
        assert!(attrib_request.is_err());
    }

    #[test]
    fn build_attrib_request_works_for_hash_field() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let hash = "some_hash";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"100","dest":"some_dest","hash":"some_hash"}"#;

        let attrib_request = ledger_service.build_attrib_request(identifier, dest, Some(hash), None, None);
        assert!(attrib_request.is_ok());
        let attrib_request = attrib_request.unwrap();
        assert!(attrib_request.contains(expected_result));
    }

    #[test]
    fn build_get_attrib_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let raw = "some_raw";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"104","dest":"some_dest","raw":"some_raw"}"#;

        let get_attrib_request = ledger_service.build_get_attrib_request(identifier, dest, raw);
        assert!(get_attrib_request.is_ok());
        let get_attrib_request = get_attrib_request.unwrap();
        assert!(get_attrib_request.contains(expected_result));
    }

    #[test]
    fn build_schema_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let data = r#"{"name":"name"}"#;

        let get_attrib_request = ledger_service.build_schema_request(identifier, data);
        assert!(get_attrib_request.is_err());
    }

    #[test]
    fn build_schema_request_works_for_correct_data() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let data = r#"{"name":"name", "version":"1.0", "keys": ["name", "male"]}"#;

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"101","data":{"name":"name","version":"1.0","keys":["name","male"]}}"#;

        let schema_request = ledger_service.build_schema_request(identifier, data);
        assert!(schema_request.is_ok());
        let schema_request = schema_request.unwrap();
        assert!(schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_schema_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let data = r#"{"name":"name", "keys": ["name", "male"]}"#;

        let get_schema_request = ledger_service.build_get_schema_request(identifier, data);
        assert!(get_schema_request.is_err());
    }

    #[test]
    fn build_get_schema_request_works_for_correct_data() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let data = r#"{"name":"name", "version":"1.0"}"#;

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"107","data":{"name":"name","version":"1.0"}}"#;

        let get_schema_request = ledger_service.build_get_schema_request(identifier, data);
        assert!(get_schema_request.is_ok());
        let get_schema_request = get_schema_request.unwrap();
        assert!(get_schema_request.contains(expected_result));
    }

    #[test]
    fn build_get_claim_def_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let _ref = "some_ref";
        let signature_type = "signature_type";

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"108","ref":"some_ref","signature_type":"signature_type"}"#;

        let get_claim_def_request = ledger_service.build_get_claim_def_request(identifier, _ref, signature_type);
        assert!(get_claim_def_request.is_ok());
        let get_claim_def_request = get_claim_def_request.unwrap();
        assert!(get_claim_def_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1, "alias":"some", "services": ["VALIDATOR"]}"#;

        let expected_result = r#""identifier":"some_identifier","operation":{"type":"0","dest":"some_dest","data":{"node_ip":"ip","node_port":1,"client_ip":"ip","client_port":1,"alias":"some","services":["VALIDATOR"]}}"#;

        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_ok());
        let node_request = node_request.unwrap();
        assert!(node_request.contains(expected_result));
    }

    #[test]
    fn build_node_request_works_for_wrong_data() {
        let ledger_service = LedgerService::new();
        let identifier = "some_identifier";
        let dest = "some_dest";
        let data = r#"{"node_ip":"ip", "node_port": 1, "client_ip": "ip", "client_port": 1}"#;

        let node_request = ledger_service.build_node_request(identifier, dest, data);
        assert!(node_request.is_err());
    }
}