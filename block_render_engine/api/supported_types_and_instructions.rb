require 'dtr_core'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    supported_instructions: DTRCore::SupportedAttributes::Instructions,
    supported_types: DTRCore::SupportedAttributes::Types
  }.to_json
end
