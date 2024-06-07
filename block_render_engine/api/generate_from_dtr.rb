require 'dtr_to_rust'

Handler = Proc.new do |request, response|
  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
   "body": request.body
  }.to_json
end
