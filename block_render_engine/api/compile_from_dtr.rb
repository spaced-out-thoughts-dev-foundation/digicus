require 'dtr_core'

class RequestHandler
  SUCCESS_STATUS_CODE = 200
  NO_BODY_STATUS_CODE = 401

  def initialize(request)
    @request = request
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
        format: content_format
      },
      status: success_status_code
    }.to_json
  end

  private

  def dtr_core_gem_version
    Gem.loaded_specs['dtr_core'].version
  end

  def dtr_version(request)
    JSON.parse(request.body)['dtr_version'] || dtr_core_gem_version
  end
  
  def content(request)
    JSON.parse(request.body)['content'] || ''
  end
  
  def content_format(request)
    JSON.parse(request.body)['format'] || 'unknown'
  end

  def default_response
    {
      received: {
        dtr_version: dtr_core_gem_version,
        content: '',
        format: 'unknown'
      },
      status: no_body_status_code
    }.to_json
  end
end

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = RequestHandler.response_body(request)
end
