Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
    supported_instructions: DTRCore::Instructions,
    supported_types: DTRCore::Types
  }.to_json
end
