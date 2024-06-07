require 'dtr_to_rust'
require 'dtr_core'
require 'fiddle'
require 'fiddle/import'
require 'net/http'
require 'uri'
require 'json'

def form_state(state)
  return [] if state.nil?
  state.map do |s|
    puts "[DEBUG]: s name: #{s["name"]}"
    puts "[DEBUG]: s type: #{s["type"]}"
    puts "[DEBUG]: s initial_value: #{s["initial_value"]}"

    DTRCore::State.new(
      s["name"],
      s["type"],
      s["initial_value"]
    )
  end
end

def form_functions(functions)
  return [] if functions.nil?

  functions.map do |f|
    puts "[DEBUG]: f name: #{f["name"]}"
    puts "[DEBUG]: f inputs: #{f["inputs"]}"
    puts "[DEBUG]: f outputs: #{f["outputs"]}"

    DTRCore::Function.new(
      f["name"],
      f["inputs"].map{ |i| i.transform_keys(&:to_sym) },
      f["outputs"],
      f["instructions"].map{ |i| i.transform_keys(&:to_sym) },
    )
  end
end

def form_user_defined_types(user_defined_types)
  return []
end

Handler = Proc.new do |request, response|
  begin
    name = JSON.parse(request.body)['contract_name']
    state = form_state(JSON.parse(request.body)['contract_state'])
    functions = form_functions(JSON.parse(request.body)['contract_functions'])
    user_defined_types = form_user_defined_types(JSON.parse(request.body)['contract_user_defined_types'])

    puts "Received request to generate Rust code from DTR code: #{request.body}"
    puts "[Debug]: contract_name: #{name}"
    puts "[Debug]: contract_state: #{state}"
    puts "[Debug]: contract_functions: #{functions}"
    puts "[Debug]: contract_user_defined_types: #{user_defined_types}"

    contract = DTRCore::Contract.new(
      name,
      state,
      functions,
      user_defined_types
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
