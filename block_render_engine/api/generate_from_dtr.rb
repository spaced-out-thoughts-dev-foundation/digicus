require 'dtr_to_rust'

Handler = Proc.new do |request, response|
  dtr_code = <<~DTR
[Contract]: AnswerToLifeContract

[InternalFunctions]:
  -() [fourty_two]
  * Inputs:
  {
    env: Env
  }
  * Output: u32
  * Instructions:
    $
      { instruction: Return, input: (42), scope: 0 }
    $
  :[InternalFunctions]
  [User Defined Types]:
  :[User Defined Types]
  DTR

  response.status = 200
  response['Content-Type'] = 'text/text; charset=utf-8'
  response.body = {
   "body": DTRToRust::Generator.generate_from_string(dtr_code)
  }.to_json
end
