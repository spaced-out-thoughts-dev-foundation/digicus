# frozen_string_literal: true

require 'spec_helper'

RSpec.describe 'Instruction Combinations' do
  context 'when method invocation chain' do
    let(:minimal_dtr_code) do
      <<~DTR
        [Contract]: MyContract

        [Interface]:
          -() [hello_world]
            * Instructions:
            $
              { instruction: evaluate, input: (foo, 10), assign: EVALUATE_RESULT_A, scope: 0 }
              { instruction: evaluate, input: (EVALUATE_RESULT_A.bar), assign: EVALUATE_RESULT_B, scope: 0 }
              { instruction: evaluate, input: (EVALUATE_RESULT_B. baz, 10, "hello", false), scope: 0 }
            $


        :[Interface]

      DTR
    end

    let(:expected_rust_code) do
      <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl};

        #[contract]
        pub struct MyContract;

        #[contractimpl]
        impl MyContract {
          pub fn hello_world() {
            foo(10).bar().baz(10, "hello", false);
          }
        }
      RUST
    end

    it 'returns the optimized instructions' do
      actual = DTRToRust::Generator.generate_from_string(minimal_dtr_code).gsub("\n", '').gsub("\t", '').gsub(' ', '')
      expected = expected_rust_code.gsub("\n", '').gsub("\t", '').gsub(' ', '')

      expect(actual).to eq(expected)
    end
  end

  context 'when if statement' do
    let(:minimal_dtr_code) do
      <<~DTR
        [Contract]: LogIfAnswerToLife

        [Interface]:
          -() [fourty_two_and_then_some]
          * Inputs:
          {
            env: Env
            possibly_the_answer_to_life: Integer
          }
          * Instructions:
          $
            { instruction: evaluate, input: (equal_to, possibly_the_answer_to_life, ANSWER_TO_LIFE), assign: UNARY_ARGUMENT_0, scope: 0 }
            { instruction: evaluate, input: (!, UNARY_ARGUMENT_0), assign: CONDITIONAL_JUMP_ASSIGNMENT, scope: 0 }
            { instruction: jump, input: (CONDITIONAL_JUMP_ASSIGNMENT, 1), scope: 0 }
            { instruction: evaluate, input: (log_to_env, env, "Yes, the answer to life is 42!"), scope: 1 }
          $
        :[Interface]

        [State]:
          * [ANSWER_TO_LIFE]
            * Type: Integer
            * Initial Value: 42
        :[State]

        [Helpers]:
          -() [log_to_env]
          * Inputs:
          {
            env: Env
            message: String
          }
          * Instructions:
          $
            { instruction: print, input: (env, message), scope: 0 }
          $
        :[Helpers]
      DTR
    end

    let(:expected_rust_code) do
      <<~RUST
        #![no_std]
        use soroban_sdk::{contract, contractimpl, Env, log, Symbol};

        #[contract]
        pub struct LogIfAnswerToLife;

        const ANSWER_TO_LIFE: i64 = 42;

        #[contractimpl]
        impl LogIfAnswerToLife {
            pub fn fourty_two_and_then_some(env: Env, possibly_the_answer_to_life: i64) {
            let CONDITIONAL_JUMP_ASSIGNMENT = !(&possibly_the_answer_to_life == &ANSWER_TO_LIFE);
            if CONDITIONAL_JUMP_ASSIGNMENT {
                log_to_env(&env, "Yes, the answer to life is 42!");
            }
        }

        fn log_to_env(env: Env, message: String) {
            log!(&env, &message);
        }

      RUST
    end

    it 'returns the optimized instructions' do
      actual = DTRToRust::Generator.generate_from_string(minimal_dtr_code).gsub("\n", '').gsub("\t", '').gsub(' ', '')
      expected = expected_rust_code.gsub("\n", '').gsub("\t", '').gsub(' ', '')
      expect(actual).to eq(expected)
    end
  end
end
