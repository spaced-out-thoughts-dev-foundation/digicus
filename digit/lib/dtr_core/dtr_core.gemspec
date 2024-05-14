# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name          = 'dtr_core'
  spec.version       = '0.2.4'
  spec.authors = ['Rob Durst']
  spec.email         = ['me@robdurst.com']
  spec.summary       = 'Core smart contract intermediate language (Digicus Textual Representation) parser.'
  spec.description   = 'Core smart contract intermediate language (Digicus Textual Representation) parser.'
  spec.homepage      = 'https://spaced-out-thoughts-dev-foundation.github.io/digicus/'
  spec.license       = 'MIT'

  # Add dependencies
  # spec.add_dependency "some_other_gem", "~> 2.0"
  # spec.add_development_dependency "rspec", "~> 3.0"

  # Specify the files to be included in the gem
  spec.files         = Dir.glob(File.join('lib', '**', '*.rb'))

  # Specify the main file to be loaded when the gem is required
  # spec.require_paths = ["lib/contract.rb"]

  # Optionally specify executables to be installed when the gem is installed
  # spec.executables   = ["my_gem_executable"]

  # Optionally specify runtime and development dependencies
  spec.required_ruby_version = '>= 3.2.0'
  # spec.required_rubygems_version = ">= 3.0.0"

  # Optionally specify additional metadata, such as platform compatibility
  spec.platform = Gem::Platform::RUBY
  # spec.metadata      = { "foo" => "bar" }
  spec.metadata['rubygems_mfa_required'] = 'true'
end
