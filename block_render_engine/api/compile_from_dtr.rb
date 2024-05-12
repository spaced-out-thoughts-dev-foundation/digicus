require 'dtr_core'


def no_body_status_code
  401
end

def success_status_code
  200
end

def dtr_core_gem_version
  Gem.loaded_specs['dtr_core'].version
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

def dtr_version(request)
  JSON.parse(request.body)['dtr_version'] || dtr_core_gem_version
end

def content(request)
  JSON.parse(request.body)['content'] || ''
end

def content_format(request)
  JSON.parse(request.body)['format'] || 'unknown'
end

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = default_response

  next unless request.body 

  response.body = {
    received: {
      dtr_version: dtr_version(request),
      content: content(request),
      format: content_format(request)
    },
    status: success_status_code
  }.to_json
end
