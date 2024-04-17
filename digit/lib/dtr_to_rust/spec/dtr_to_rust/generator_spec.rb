# frozen_string_literal: true

require './spec/spec_helper'

RSpec.describe DTRToRust::Generator do
  context 'when generating rust for the simple hello world contract' do
    let(:file_path) { 'spec/test_dtr_files/hello_world.dtr' }
    let(:generator) { described_class.new(file_path) }

    let(:expected_content) do
      <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

        #[contract]
        pub struct HelloContract;

        #[contractimpl]
        impl HelloContract {
            pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
                vec![&env, symbol_short!("Hello"), to]
            }
        }
      RUST
    end

    it 'generates valid Rust' do
      expect(generator.generate).to eq(expected_content)
    end
  end
end
