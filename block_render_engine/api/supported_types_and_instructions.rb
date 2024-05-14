require 'dtr_core'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    supported_instructions: DTRCore::SupportedAttributes::INSTRUCTIONS,
    supported_types: DTRCore::SupportedAttributes::TYPES
  }.to_json
end
