# frozen_string_literal: true

# Core logic for consuming Digicus Textual Representation (DTR) files.
module DTRCore
  autoload :FileNotFoundError, 'dtr_core/file_not_found_error'

  autoload :Parser, 'dtr_core/parser'
  autoload :State, 'dtr_core/state'
  autoload :Contract, 'dtr_core/contract'
end
