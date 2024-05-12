require 'dtr_core'

class RequestHandler
  SUCCESS_STATUS_CODE = 200
  NO_BODY_STATUS_CODE = 401
  FAILED_TO_COMPILE_STATUS_CODE = 402

  def initialize(request)
    @request = request

    @contract_name = "Unknown"

    @compilation_error = ''
  end

  def self.response_body(request)
    new(request).response_body
  end

  def response_body
    return default_response unless @request.body

    {
      received: {
        dtr_version: dtr_version,
        content: content,
        format: content_format,
        contract_name: @contract_name,
        compilation_error: @compilation_error
      },
      status: status
    }.to_json
  end

  private

  def status
    begin
      contract = DTRCore::Contract.from_filepath('../hello_world.dtr')

      @contract_name = contract.name

      SUCCESS_STATUS_CODE
    rescue _ => e
      @compilation_error = e

      FAILED_TO_COMPILE_STATUS_CODE
    end
  end

  def dtr_core_gem_version
    Gem.loaded_specs['dtr_core'].version
  end

  def dtr_version
    JSON.parse(@request.body)['dtr_version'] || dtr_core_gem_version
  end
  
  def content
    JSON.parse(@request.body)['content'] || ''
  end
  
  def content_format
    JSON.parse(@request.body)['format'] || 'unknown'
  end

  def default_response
    {
      received: {
        dtr_version: dtr_core_gem_version,
        content: '',
        format: 'unknown'
      },
      status: NO_BODY_STATUS_CODE
    }.to_json
  end
end

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = RequestHandler.response_body(request)
end
