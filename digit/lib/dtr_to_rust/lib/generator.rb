require 'dtr_core'

module DTRToRust
  class Generator
    def initialize(file_path)
      @file_path = file_path
      @dtr_contract = ::DTRCore::Parser.parse(file_path)
    end

    def generate
      generate_contract_header
      generate_contract_name
      generate_state
      generate_functions
    end

    private

    attr_reader :dtr_contract

    def generate_contract_header
      puts "#![no_std]\nuse soroban_sdk::{contract, contractimpl};\n\n"
    end

    def generate_contract_name
      puts "#[contract]\npub struct #{dtr_contract.name};\n\n"
    end

    def generate_state
      return if dtr_contract.state.nil?

      puts "pub struct State {"
      dtr_contract.state.each do |state|
        puts "  pub #{state.name}: #{state.type},"
      end
      puts "}\n\n"
    end

    def generate_functions
      puts "#[contractimpl]\nimpl #{dtr_contract.name} { }\n"
    end
  end
end
