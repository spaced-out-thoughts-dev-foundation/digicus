require 'dtr_core'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'

  unless request.body 
    response.body = {
      received: {
        dtr_version: Gem.loaded_specs['dtr_core'].version,
        content: 'Hello, World!'
      },
      status: 401
    }.to_json

    next
  end

  dtr_version = JSON.parse(request.body)['dtr_version'] || Gem.loaded_specs['dtr_core'].version
  content = JSON.parse(request.body)['content'] || ''

  response.body = {
    received: {
      dtr_version: dtr_version,
      content: content
    },
    status: 200
  }.to_json
end
