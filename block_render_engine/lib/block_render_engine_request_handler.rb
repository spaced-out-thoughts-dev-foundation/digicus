require 'fiddle'
require 'fiddle/import'
require 'net/http'
require 'uri'
require 'json'

# require 'rust_to_dtr/version'

class BlockRenderEngineRequestHandler
  SUCCESS_STATUS_CODE = 200
  NO_BODY_STATUS_CODE = 401
  FAILED_TO_COMPILE_STATUS_CODE = 402
  FAILED_TO_TRANSPILE_STATUS_CODE = 412

  def initialize(request)
    @request = request
  end

  def self.response_body(request)
    new(request).response_body
  end

  def response_body
    @last_method_executed = 'response_body'
    return default_response unless @request.body && (dtr? || rust?)

    compile

    if rust?
      url = URI.parse('https://temprustfunctions-git-main-robertdursts-projects.vercel.app/api/handler')
      http = Net::HTTP.new(url.host, url.port)
      http.use_ssl = true  # Use SSL/TLS for the request
      request = Net::HTTP::Post.new(url.path, {'Content-Type' => 'application/json', 'Accept' => 'application/json'})
      request_body = {
        content: content,
      }

      # Convert the request body to JSON
      request.body = request_body.to_json

      # Send the request and get the response
      response = http.request(request)

      # Print the response body
      puts "Response Code: #{response.code}"
      puts "Response Body: #{response.body}"
    # else
      # @transpiled_code = content
    end

    
    {
      received: {
        content: content,
        format: content_format,
      },
      contract_name: @contract_name,
      contract_state: @contract_state,
      contract_functions: @contract_functions,
      compilation_error: @compilation_error,
      content_final: content,
      status: status,
      last_method_executed: @last_method_executed,
    }.to_json
  end

  private

  def dtr?
    @last_method_executed = 'dtr?'

    content_format == 'dtr'
  end

  def rust?
    @last_method_executed = 'rust?'

    content_format == 'rust'
  end

  def compile
    @last_method_executed = 'compile'
     begin
      contract = DTRCore::Contract.from_dtr_raw(content)

      @contract_name = contract.name
      @contract_state = contract.state&.map do |s|
        { 
          name: s.name,
          type: s.type,
          initial_value: s.initial_value
        }.to_json
      end
      @contract_functions = contract.functions&.map do |f|
        { 
          name: f.name,
          instructions: f.instructions&.map { |x| x[:instruction] }.join(' ')
        }.to_json
      end

      @compilation_success = true
      @compilation_error = ''
    rescue StandardError => e
      @contract_name = "Unknown"
      @contract_state = []
      @contract_functions = []

      @compilation_error = e
    end
  end

  def status
    @last_method_executed = 'status'
   @compilation_success ? SUCCESS_STATUS_CODE : FAILED_TO_COMPILE_STATUS_CODE
  end

  def dtr_core_gem_version
    @last_method_executed = 'dtr_core_gem_version'
    Gem.loaded_specs['dtr_core'].version
  end
  
  def content
    @last_method_executed = 'content'
    JSON.parse(@request.body)['content'].gsub('\n', "\n") || ''
  end
  
  def content_format
    @last_method_executed = 'content_format'
    JSON.parse(@request.body)['format'] || 'unknown'
  end

  def default_response
    @last_method_executed = 'default_response'
    { status: NO_BODY_STATUS_CODE }.to_json
  end
end
