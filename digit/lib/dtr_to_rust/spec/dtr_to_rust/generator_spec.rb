# frozen_string_literal: true

require 'spec_helper'

RSpec.describe DTRToRust::Generator do
  let(:minimal_dtr_code) do
    <<~DTR
        [Contract]: MyContract

        [State]:
      * [COUNTER]
        * Type: String
        * Initial Value: "COUNTER"
        :[State]

    DTR
  end

  let(:expected_rust_code) do
    <<~RUST
      #![no_std]
      use soroban_sdk::{Symbol, symbol_short, contract, contractimpl};

      const COUNTER: Symbol = symbol_short!("COUNTER");

      #[contract]
      pub struct MyContract;

      #[contractimpl]
      impl MyContract {}
    RUST
  end

  describe '#generate_from_string' do
    it 'generates Rust code from a DTR contract' do
      actual = described_class.generate_from_string(minimal_dtr_code).gsub("\n", '').gsub("\t", '')
      expected = expected_rust_code.gsub("\n", '').gsub("\t", '')

      expect(actual).to eq(expected)
    end
  end

  describe '#generate' do
    it 'generates Rust code from a DTR contract' do
      minimal_dtr_code = <<~DTR
        [Contract]: MyContract

        [State]:
          * [COUNTER]
            * Type: String
            * Initial Value: "COUNTER"
        :[State]

      DTR
      minimal_dtr_file_path = 'spec/temp.dtr'

      FileUtils.rm_f(minimal_dtr_file_path)
      File.write(minimal_dtr_file_path, minimal_dtr_code)

      actual = described_class.generate_from_string(minimal_dtr_code).gsub("\n", '').gsub("\t",
                                                                                          '')
      expected = expected_rust_code.gsub("\n", '').gsub("\t", '')

      expect(actual).to eq(expected)
    end
  end
end
