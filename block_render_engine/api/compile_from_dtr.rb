require 'dtr_core'

class RequestHandler
  SUCCESS_STATUS_CODE = 200
  NO_BODY_STATUS_CODE = 401
  FAILED_TO_COMPILE_STATUS_CODE = 402

  def initialize(request)
    @request = request
  end

  def self.response_body(request)
    new(request).response_body
  end

  def response_body
    return default_response unless @request.body

    compile

    {
      received: {
        content: content,
        format: content_format,
        contract_name: @contract_name,
        compilation_error: @compilation_error
      },
      status: status
    }.to_json
  end

  private

  def compile
     begin
      contract = DTRCore::Contract.from_dtr_raw(content)

      @contract_name = contract.name

      @compilation_success = true
      @compilation_error = ''
    rescue StandardError => e
      @compilation_error = e
      @contract_name = "Unknown"
    end
  end

  def status
   @compilation_success ? SUCCESS_STATUS_CODE : FAILED_TO_COMPILE_STATUS_CODE
  end

  def dtr_core_gem_version
    Gem.loaded_specs['dtr_core'].version
  end
  
  def content
    JSON.parse(@request.body)['content'] || ''
  end
  
  def content_format
    JSON.parse(@request.body)['format'] || 'unknown'
  end

  def default_response
    { status: NO_BODY_STATUS_CODE }.to_json
  end
end

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = RequestHandler.response_body(request)
end
