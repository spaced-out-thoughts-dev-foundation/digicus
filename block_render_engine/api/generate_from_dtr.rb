require 'dtr_to_rust'
require 'dtr_core'
require 'fiddle'
require 'fiddle/import'
require 'net/http'
require 'uri'
require 'json'

Handler = Proc.new do |request, response|
  begin
    puts "Received request to generate Rust code from DTR code: #{request.body}"
    puts "[Debug]: contract_name: #{JSON.parse(request.body)['contract_name']}"
    puts "[Debug]: contract_state: #{JSON.parse(request.body)['contract_state']}"
    puts "[Debug]: contract_functions: #{JSON.parse(request.body)['contract_functions']}"
    puts "[Debug]: contract_user_defined_types: #{JSON.parse(request.body)['contract_user_defined_types']}"

    contract = DTRCore::Contract.new(
      JSON.parse(request.body)['contract_name'],
      JSON.parse(request.body)['contract_state'],
      JSON.parse(request.body)['contract_functions'],
      JSON.parse(request.body)['contract_user_defined_types']
    )

    puts "[DEBUG]: formed contract"

    dtr_code = contract.to_s

    puts "[DEBUG]: generated DTR code: #{dtr_code}"

    rust_code = DTRToRust::Generator.generate_from_string(dtr_code)

    puts "[DEBUG]: generated Rust code"

    response.status = 200
    response['Content-Type'] = 'text/text; charset=utf-8'
    response.body = {
      "dtr_code": dtr_code,
      "rust_code": rust_code
    }.to_json
  rescue => e
    puts "[ERROR]: #{e.message}"
    response.status = 500
    response['Content-Type'] = 'text/text; charset=utf-8'
    response.body = {
      "error": e.message
    }.to_json
  end
end
