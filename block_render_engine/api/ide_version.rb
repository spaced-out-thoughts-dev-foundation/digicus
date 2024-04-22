require 'dtr_core'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = Gem.loaded_specs['dtr_core'].version
end
