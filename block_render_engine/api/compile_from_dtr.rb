require 'dtr_core'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    dtr_version: request.params['dtr_version'] || Gem.loaded_specs['dtr_core'].version
  }.to_json
end
