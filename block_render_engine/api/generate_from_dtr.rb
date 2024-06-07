require 'dtr_to_rust'
require 'dtr_core'
require 'fiddle'
require 'fiddle/import'
require 'net/http'
require 'uri'
require 'json'

Handler = Proc.new do |request, response|
  begin
    contract = DTRCore::Contract.new(
      request.body.contract.contract_name,
      request.body.contract.contract_state,
      request.body.contract.contract_functions,
      request.body.contract.contract_user_defined_types
    )

    dtr_code = contract.to_s
    rust_code = DTRToRust::DtrToRust::Generator.generate_from_string(dtr_code)

    response.status = 200
    response['Content-Type'] = 'text/text; charset=utf-8'
    response.body = {
      "dtr_code": dtr_code,
      "rust_code": rust_code
    }.to_json
  rescue => e
    response.status = 500
    response['Content-Type'] = 'text/text; charset=utf-8'
    response.body = {
      "error": e.message
    }.to_json
  end
end
