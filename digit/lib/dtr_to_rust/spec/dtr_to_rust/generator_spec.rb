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
      use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec, log};

      const COUNTER: Symbol = symbol_short!("COUNTER");

      #[contract]
      pub struct MyContract;


      #[contractimpl]
      impl MyContract {}
    RUST
  end

  describe '#generate_from_string' do
    it 'generates Rust code from a DTR contract' do
      expect(described_class.generate_from_string(minimal_dtr_code)).to eq(expected_rust_code)
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

      expect(described_class.generate_from_file(minimal_dtr_file_path)).to eq(expected_rust_code)
    end
  end
end
