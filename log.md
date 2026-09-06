cargo : warning: unused variable: `ppu`
At line:1 char:1
+ cargo test --lib -- --nocapture *> log.md
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : NotSpecified: (warning: unused variable: `ppu`:String) [], RemoteException
    + FullyQualifiedErrorId : NativeCommandError
 
   --> src\game\game\hardware\ppu_tests.rs:295:9
    |
295 |     let ppu = Ppu::new();
    |         ^^^ help: if this is intentional, prefix it with an underscore: `_ppu`
    |
    = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: struct `BgAttributes` is never constructed
  --> src\game\game\hardware\ppu.rs:30:8
   |
30 | struct BgAttributes {
   |        ^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `from_byte` is never used
  --> src\game\game\hardware\ppu.rs:40:8
   |
38 | impl BgAttributes {
   | ----------------- associated function in this implementation
39 |     #[inline]
40 |     fn from_byte(value: u8) -> Self {
   |        ^^^^^^^^^

warning: method `bg_map_base` is never used
   --> src\game\game\hardware\ppu.rs:192:8
    |
 50 | impl Ppu {
    | -------- method in this implementation
...
192 |     fn bg_map_base(&self) -> u16 {
    |        ^^^^^^^^^^^

warning: `gameboy-port` (lib test) generated 4 warnings (run `cargo fix --lib -p gameboy-port --tests` to apply 1 suggestion)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src\game\lib.rs (target\debug\deps\gameboy_port-5d21912cd8e92f42.exe)

running 151 tests
test game::cpu::tests::adc_carry_without_half_carry ... ok
test game::cpu::tests::adc_half_and_full_carry ... ok
test game::cpu::tests::adc_half_carry_from_carry_flag ... ok
test game::cpu::tests::adc_half_carry_only ... ok
test game::cpu::tests::adc_without_initial_carry ... ok
test game::cpu::tests::adc_zero_and_full_carry ... ok
test game::cpu::tests::adc_with_initial_carry ... ok
test game::cpu::tests::adc_preserves_no_flags_when_not_needed ... ok
test game::cpu::tests::add_carry_without_half_carry ... ok
test game::cpu::tests::add_full_carry_and_zero ... ok
test game::cpu::tests::add_half_carry ... ok
test game::cpu::tests::add_half_carry_another_case ... ok
test game::cpu::tests::add_simple ... ok
test game::cpu::tests::add_upper_nibble_carry_only ... ok
test game::cpu::tests::add_zero_sets_z ... ok
test game::cpu::tests::cp_clears_old_flags ... ok
test game::cpu::tests::cp_does_not_modify_a ... ok
test game::cpu::tests::cp_equal ... ok
test game::cpu::tests::cp_ff_equal_ff ... ok
test game::cpu::tests::cp_full_borrow ... ok
test game::cpu::tests::cp_greater_than_operand ... ok
test game::cpu::tests::cp_half_borrow ... ok
test game::cpu::tests::cp_less_than_operand ... ok
test game::cpu::tests::cpu_cycles_drive_timer ... ok
test game::cpu::tests::halt_bug_reuses_next_opcode_byte ... ok
test game::cpu::tests::di_disables_ime_and_cancels_pending_ei ... ok
test game::cpu::tests::halt_wakes_on_pending_interrupt ... ok
test game::cpu::tests::halt_wakes_on_pending_interrupt_without_ime ... ok
test game::cpu::tests::call_ret ... ok
test game::cpu::tests::conditional_call_all_conditions ... ok
test game::cpu::tests::conditional_call_ret ... ok
test game::cpu::tests::ei_enables_ime_after_next_instruction ... ok
test game::cpu::tests::ei_delays_interrupt_until_after_next_instruction ... ok
test game::cpu::tests::halt_stops_cpu ... ok
test game::cpu::tests::sbc_borrow_with_ff ... ok
test game::cpu::tests::sbc_full_borrow_from_carry ... ok
test game::cpu::tests::sbc_full_borrow_operand ... ok
test game::cpu::tests::sbc_half_borrow_boundary ... ok
test game::cpu::tests::sbc_half_borrow_from_carry ... ok
test game::cpu::tests::sbc_carry_and_half_carry_together ... ok
test game::cpu::tests::interrupt_priority_vblank_over_timer ... ok
test game::cpu::tests::interrupt_priority_all_vectors ... ok
test game::cpu::tests::sbc_result_fe ... ok
test game::cpu::tests::sbc_zero_without_borrow ... ok
test game::cpu::tests::interrupt_services_timer_when_ime_enabled ... ok
test game::cpu::tests::push_af_pop_af ... ok
test game::cpu::tests::sbc_with_input_carry ... ok
test game::cpu::tests::sub_full_borrow ... ok
test game::cpu::tests::sbc_without_carry ... ok
test game::cpu::tests::sbc_zero_with_operand_and_carry ... ok
test game::cpu::tests::sub_full_borrow_without_half_borrow ... ok
test game::cpu::tests::reti_restores_pc_and_enables_ime ... ok
test game::cpu::tests::sub_half_borrow ... ok
test game::cpu::tests::sub_half_borrow_boundary ... ok
test game::cpu::tests::sub_half_borrow_lower_nibble ... ok
test game::cpu::tests::sub_no_half_no_carry ... ok
test game::cpu::tests::sub_simple ... ok
test game::cpu::tests::sub_zero ... ok
test game::cpu::tests::sub_zero_from_ff ... ok
test game::cpu::tests::interrupt_then_reti_restores_cpu_state ... ok
test game::cpu::tests::rst_all_vectors ... ok
test game::cpu::tests::push_bc_pop_bc ... ok
test game::cpu::tests::joypad_interrupt_jumps_to_0060 ... ok
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
test game::cpu_base_00_3f_tests::base_00_3f_flags_and_special_cases ... ok
test game::cpu_base_00_3f_tests::base_00_3f_conditional_jr_both_paths ... ok
test game::cpu_base_00_3f_tests::base_opcodes_00_3f_cycles_and_pc ... ok
test game::cpu_base_40_7f_tests::base_40_7f_hl_loads_use_memory_operand ... ok
test game::cpu_base_40_7f_tests::base_40_7f_all_ld_opcodes_have_correct_cycles_and_pc ... ok
test game::cpu_base_40_7f_tests::base_40_7f_ld_hl_hl_is_noop_and_halt_is_covered_by_existing_test ... ok
test game::cpu_base_40_7f_tests::base_40_7f_ld_register_matrix_copies_correct_values ... ok
test game::cpu_base_40_7f_tests::base_40_7f_register_to_hl_stores_correct_values ... ok
test game::cpu_base_00_3f_tests::base_00_3f_memory_and_pair_instructions ... ok
test game::cpu_base_80_bf_tests::base_80_bf_hl_operand_is_used_and_flags_are_correct ... ok
test game::cpu_base_80_bf_tests::base_80_bf_non_a_registers_are_not_modified ... ok
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
test game::cpu::tests::interrupt_all_vectors_individually ... ok
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU BG: screen=(000,001) source=(000,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=1 px=0 color_id=0
PPU BG: screen=(001,001) source=(001,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=1 px=1 color_id=0
PPU BG: screen=(002,001) source=(002,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,001) row=1 px=2 color_id=0
PPU BG: screen=(003,001) source=(003,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,001) row=1 px=3 color_id=0
PPU BG: screen=(004,001) source=(004,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=1 px=4 color_id=0
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(005,001) source=(005,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=1 px=5 color_id=0
test game::cpu_base_c0_ff_tests::base_c0_ff_di_ei_and_sp_hl_special_cases_are_correct ... ok
PPU BG: screen=(006,001) source=(006,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=1 px=6 color_id=0
PPU BG: screen=(007,001) source=(007,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=1 px=7 color_id=0
PPU BG: screen=(008,001) source=(008,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=1 px=0 color_id=0
PPU BG: screen=(009,001) source=(009,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=1 px=1 color_id=0
PPU BG: screen=(010,001) source=(010,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=1 px=2 color_id=0
PPU BG: screen=(011,001) source=(011,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
test game::cpu_base_c0_ff_tests::base_c0_ff_call_ret_and_rst_preserve_return_address ... ok
test game::cpu_base_80_bf_tests::base_80_bf_logic_flags_are_correct ... ok
PPU PIXEL: screen=(011,001) row=1 px=3 color_id=0
PPU BG: screen=(012,001) source=(012,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=1 px=4 color_id=0
PPU BG: screen=(013,001) source=(013,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=1 px=5 color_id=0
PPU BG: screen=(014,001) source=(014,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=1 px=6 color_id=0
PPU BG: screen=(015,001) source=(015,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=1 px=7 color_id=0
PPU BG: screen=(016,001) source=(016,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,001) row=1 px=0 color_id=0
PPU BG: screen=(017,001) source=(017,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=1 px=1 color_id=0
PPU BG: screen=(018,001) source=(018,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=1 px=2 color_id=0
PPU BG: screen=(019,001) source=(019,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=1 px=3 color_id=0
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(000,002) source=(000,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,002) row=2 px=0 color_id=0
PPU BG: screen=(001,002) source=(001,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,002) row=2 px=1 color_id=0
PPU BG: screen=(002,002) source=(002,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,002) row=2 px=2 color_id=0
PPU BG: screen=(003,002) source=(003,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,001) source=(000,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=1 px=0 color_id=0
PPU BG: screen=(001,001) source=(001,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=1 px=1 color_id=0
PPU BG: screen=(002,001) source=(002,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,001) row=1 px=2 color_id=0
PPU BG: screen=(003,001) source=(003,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,001) row=1 px=3 color_id=0
PPU BG: screen=(004,001) source=(004,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=1 px=4 color_id=0
PPU BG: screen=(005,001) source=(005,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=1 px=5 color_id=0
PPU BG: screen=(006,001) source=(006,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=1 px=6 color_id=0
PPU BG: screen=(007,001) source=(007,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=1 px=7 color_id=0
PPU BG: screen=(008,001) source=(008,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=1 px=0 color_id=0
PPU BG: screen=(009,001) source=(009,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=1 px=1 color_id=0
PPU BG: screen=(010,001) source=(010,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=1 px=2 color_id=0
PPU BG: screen=(011,001) source=(011,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,001) row=1 px=3 color_id=0
PPU BG: screen=(012,001) source=(012,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=1 px=4 color_id=0
PPU BG: screen=(013,001) source=(013,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=1 px=5 color_id=0
PPU BG: screen=(014,001) source=(014,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=1 px=6 color_id=0
PPU BG: screen=(015,001) source=(015,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=1 px=7 color_id=0
PPU BG: screen=(016,001) source=(016,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,002) row=2 px=3 color_id=0
PPU PIXEL: screen=(016,001) row=1 px=0 color_id=0
PPU BG: screen=(017,001) source=(017,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=1 px=1 color_id=0
PPU BG: screen=(018,001) source=(018,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=1 px=2 color_id=0
PPU BG: screen=(019,001) source=(019,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=1 px=3 color_id=0
PPU BG: screen=(004,002) source=(004,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,002) source=(000,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,002) row=2 px=0 color_id=0
PPU BG: screen=(001,002) source=(001,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,002) row=2 px=1 color_id=0
PPU BG: screen=(002,002) source=(002,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,002) row=2 px=2 color_id=0
PPU BG: screen=(003,002) source=(003,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,002) row=2 px=3 color_id=0
PPU BG: screen=(004,002) source=(004,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,002) row=2 px=4 color_id=0
PPU BG: screen=(005,002) source=(005,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,002) row=2 px=5 color_id=0
PPU BG: screen=(006,002) source=(006,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,002) row=2 px=6 color_id=0
PPU BG: screen=(007,002) source=(007,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,002) row=2 px=7 color_id=0
PPU BG: screen=(008,002) source=(008,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,002) row=2 px=0 color_id=0
PPU BG: screen=(009,002) source=(009,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,002) row=2 px=1 color_id=0
PPU BG: screen=(010,002) source=(010,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,002) row=2 px=2 color_id=0
PPU BG: screen=(011,002) source=(011,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,002) row=2 px=3 color_id=0
PPU BG: screen=(012,002) source=(012,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,002) row=2 px=4 color_id=0
PPU BG: screen=(013,002) source=(013,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,002) row=2 px=4 color_id=0
PPU BG: screen=(005,002) source=(005,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,002) row=2 px=5 color_id=0
PPU BG: screen=(006,002) source=(006,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,002) row=2 px=6 color_id=0
PPU BG: screen=(007,002) source=(007,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,002) row=2 px=7 color_id=0
PPU BG: screen=(008,002) source=(008,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,002) row=2 px=0 color_id=0
PPU BG: screen=(009,002) source=(009,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,002) row=2 px=1 color_id=0
PPU BG: screen=(010,002) source=(010,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,002) row=2 px=2 color_id=0
PPU BG: screen=(011,002) source=(011,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,002) row=2 px=3 color_id=0
PPU BG: screen=(012,002) source=(012,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,002) row=2 px=4 color_id=0
PPU BG: screen=(013,002) source=(013,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,002) row=2 px=5 color_id=0
PPU BG: screen=(014,002) source=(014,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,002) row=2 px=6 color_id=0
PPU BG: screen=(015,002) source=(015,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,002) row=2 px=7 color_id=0
PPU BG: screen=(016,002) source=(016,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,002) row=2 px=0 color_id=0
PPU BG: screen=(017,002) source=(017,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,002) row=2 px=1 color_id=0
PPU BG: screen=(018,002) source=(018,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,002) row=2 px=2 color_id=0
PPU BG: screen=(019,002) source=(019,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,002) row=2 px=3 color_id=0
PPU PIXEL: screen=(013,002) row=2 px=5 color_id=0
PPU BG: screen=(014,002) source=(014,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,003) source=(000,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,003) row=3 px=0 color_id=0
PPU BG: screen=(001,003) source=(001,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,003) row=3 px=1 color_id=0
PPU BG: screen=(002,003) source=(002,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,003) row=3 px=2 color_id=0
PPU BG: screen=(003,003) source=(003,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,003) row=3 px=3 color_id=0
PPU BG: screen=(004,003) source=(004,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,003) row=3 px=4 color_id=0
PPU BG: screen=(005,003) source=(005,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,003) row=3 px=5 color_id=0
PPU BG: screen=(006,003) source=(006,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,003) row=3 px=6 color_id=0
PPU BG: screen=(007,003) source=(007,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,003) row=3 px=7 color_id=0
PPU BG: screen=(008,003) source=(008,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,003) row=3 px=0 color_id=0
PPU BG: screen=(009,003) source=(009,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,003) row=3 px=1 color_id=0
PPU BG: screen=(010,003) source=(010,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,003) row=3 px=2 color_id=0
PPU BG: screen=(011,003) source=(011,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,003) row=3 px=3 color_id=0
PPU BG: screen=(012,003) source=(012,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,003) row=3 px=4 color_id=0
PPU BG: screen=(013,003) source=(013,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,003) row=3 px=5 color_id=0
PPU BG: screen=(014,003) source=(014,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,003) row=3 px=6 color_id=0
PPU BG: screen=(015,003) source=(015,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,003) row=3 px=7 color_id=0
PPU BG: screen=(016,003) source=(016,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,003) row=3 px=0 color_id=0
PPU BG: screen=(017,003) source=(017,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,003) row=3 px=1 color_id=0
PPU BG: screen=(018,003) source=(018,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,003) row=3 px=2 color_id=0
PPU BG: screen=(019,003) source=(019,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,003) row=3 px=3 color_id=0
PPU PIXEL: screen=(014,002) row=2 px=6 color_id=0
PPU BG: screen=(015,002) source=(015,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,002) row=2 px=7 color_id=0
PPU BG: screen=(016,002) source=(016,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,002) row=2 px=0 color_id=0
PPU BG: screen=(017,002) source=(017,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,002) row=2 px=1 color_id=0
PPU BG: screen=(018,002) source=(018,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,002) row=2 px=2 color_id=0
PPU BG: screen=(019,002) source=(019,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,002) row=2 px=3 color_id=0
PPU BG: screen=(000,003) source=(000,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,003) row=3 px=0 color_id=0
PPU BG: screen=(001,003) source=(001,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,003) row=3 px=1 color_id=0
PPU BG: screen=(002,003) source=(002,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,003) row=3 px=2 color_id=0
PPU BG: screen=(003,003) source=(003,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,003) row=3 px=3 color_id=0
PPU BG: screen=(004,003) source=(004,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,003) row=3 px=4 color_id=0
PPU BG: screen=(005,003) source=(005,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,003) row=3 px=5 color_id=0
PPU BG: screen=(006,003) source=(006,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,003) row=3 px=6 color_id=0
PPU BG: screen=(007,003) source=(007,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,003) row=3 px=7 color_id=0
PPU BG: screen=(008,003) source=(008,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,003) row=3 px=0 color_id=0
PPU BG: screen=(009,003) source=(009,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,003) row=3 px=1 color_id=0
PPU BG: screen=(010,003) source=(010,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,003) row=3 px=2 color_id=0
PPU BG: screen=(011,003) source=(011,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,003) row=3 px=3 color_id=0
PPU BG: screen=(012,003) source=(012,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,003) row=3 px=4 color_id=0
PPU BG: screen=(013,003) source=(013,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,003) row=3 px=5 color_id=0
PPU BG: screen=(014,003) source=(014,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,003) row=3 px=6 color_id=0
PPU BG: screen=(015,003) source=(015,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,003) row=3 px=7 color_id=0
PPU BG: screen=(016,003) source=(016,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,003) row=3 px=0 color_id=0
PPU BG: screen=(017,003) source=(017,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,003) row=3 px=1 color_id=0
PPU BG: screen=(018,003) source=(018,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,003) row=3 px=2 color_id=0
PPU BG: screen=(019,003) source=(019,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,003) row=3 px=3 color_id=0
test game::cpu_base_80_bf_tests::base_80_bf_add_adc_flags_are_correct ... ok
test game::cpu::tests::serial_interrupt_jumps_to_0058 ... ok
test game::cpu::tests::vblank_interrupt_from_ppu_reaches_cpu ... ok
test game::cpu_base_80_bf_tests::base_80_bf_sub_sbc_cp_flags_are_correct ... ok
test game::cpu_control_tests::ei_enables_ime_after_exactly_one_following_instruction ... ok
test game::cpu_control_tests::di_cancels_pending_ei_and_disables_ime ... ok
test game::cpu_base_c0_ff_tests::base_c0_ff_stack_push_pop_and_pop_af_mask_are_correct ... ok
test game::cpu_control_tests::halt_bug_does_not_advance_pc_before_next_instruction ... ok
test game::cpu_control_tests::halt_with_ime_disabled_wakes_without_servicing_interrupt ... ok
test game::cpu_control_tests::halt_with_ime_enabled_services_pending_interrupt ... ok
test game::cpu_control_tests::interrupt_priority_selects_lowest_pending_bit ... ok
test game::cpu_control_tests::interrupt_service_clears_only_selected_if_bit ... ok
test game::cpu_control_tests::reti_restores_pc_and_enables_ime_immediately ... ok
test game::cpu_control_tests::stop_consumes_second_byte_and_enters_stopped_state ... ok
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
test game::cpu_timing_tests::cpu_cycles_forward_exactly_to_ppu_scanline_timing ... ok
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
test game::cpu_timing_tests::cpu_cycles_forward_exactly_to_timer ... ok
test game::cpu_timing_tests::cumulative_timing_of_call_and_return_is_exact ... ok
test game::hardware::interrupt::tests::clear_clears_if_bit ... ok
test game::hardware::interrupt::tests::pending_masks_ie_and_if ... ok
test game::hardware::interrupt::tests::read_if_sets_unused_bits ... ok
test game::hardware::interrupt::tests::request_sets_if_bit ... ok
test game::hardware::interrupt::tests::write_if_only_keeps_lower_five_bits ... ok
test game::hardware::joypad::tests::action_buttons_are_reported_when_action_group_is_selected ... ok
test game::hardware::joypad::tests::button_press_generates_interrupt ... ok
test game::hardware::joypad::tests::direction_and_action_groups_do_not_leak_into_each_other ... ok
test game::hardware::joypad::tests::joyp_read_write ... ok
test game::hardware::ppu::tests::background_tile_attribute_info_decodes_cgb_attribute_bits ... ok
test game::hardware::ppu::tests::background_tile_attributes_reads_vram_bank_1_at_same_map_offset ... ok
SCY TEST: y=0, scy=255, bg_y=255, tile_y=31, map=0x9800
PPU BG: screen=(000,000) source=(000,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=7 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=7 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=7 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=7 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=7 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=7 px=5 color_id=1
PPU BG: screen=(006,000) source=(006,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=7 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=7 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=7 px=0 color_id=1
PPU BG: screen=(009,000) source=(009,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=7 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=7 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=7 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=7 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=7 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=7 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=7 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=7 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=7 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=7 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=7 px=3 color_id=1
test game::cpu_timing_tests::interrupt_entry_has_fixed_20_cycle_latency ... ok
SCY TEST: y=1, scy=255, bg_y=0, tile_y=0, map=0x9800
PPU BG: screen=(000,001) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=0 px=0 color_id=2
PPU BG: screen=(001,001) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=0 px=1 color_id=2
PPU BG: screen=(002,001) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,001) row=0 px=2 color_id=2
PPU BG: screen=(003,001) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,001) row=0 px=3 color_id=2
PPU BG: screen=(004,001) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=0 px=4 color_id=2
PPU BG: screen=(005,001) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=0 px=5 color_id=2
PPU BG: screen=(006,001) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=0 px=6 color_id=2
PPU BG: screen=(007,001) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x01
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=0 px=7 color_id=2
PPU BG: screen=(008,001) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=0 px=0 color_id=1
PPU BG: screen=(009,001) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=0 px=1 color_id=1
PPU BG: screen=(010,001) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=0 px=2 color_id=1
PPU BG: screen=(011,001) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,001) row=0 px=3 color_id=1
PPU BG: screen=(012,001) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=0 px=4 color_id=1
PPU BG: screen=(013,001) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=0 px=5 color_id=1
PPU BG: screen=(014,001) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=0 px=6 color_id=1
PPU BG: screen=(015,001) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=0 px=7 color_id=1
PPU BG: screen=(016,001) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,001) row=0 px=0 color_id=1
PPU BG: screen=(017,001) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=0 px=1 color_id=1
PPU BG: screen=(018,001) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=0 px=2 color_id=1
PPU BG: screen=(019,001) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=0 px=3 color_id=1

========== SCY REGRESSION DIAGNOSTIC ==========

--- SCREEN LINE 0 ---
screen_y       = 0
SCY            = 255
bg_y           = 254
tile_x         = 0
tile_y         = 31
map_base       = 0x9800
map_index      = 0x1BE0
tile_index     = 0
attr           = 0x00
palette        = 0
bank           = false
flip_x         = false
flip_y         = false
priority       = false
actual[0]      = 0xFFACACAC
actual[159]    = 0xFFACACAC
expected       = 0xFFACACAC

--- SCREEN LINE 1 ---
screen_y       = 1
SCY            = 255
bg_y           = 0
tile_x         = 0
tile_y         = 0
map_base       = 0x9800
map_index      = 0x1800
tile_index     = 1
attr           = 0x00
palette        = 0
bank           = false
flip_x         = false
flip_y         = false
priority       = false
actual[0]      = 0xFF525252
actual[159]    = 0xFFACACAC
expected       = 0xFF525252

--- TILE DATA ---
tile 0 row 0 = [1, 1, 1, 1, 1, 1, 1, 1]
tile 1 row 0 = [2, 2, 2, 2, 2, 2, 2, 2]

===============================================


thread 'game::hardware::ppu_scroll_regression_tests::background_scroll_scy_wraps_from_line_255_to_line_0' (6060) panicked at src\game\game\hardware\ppu_scroll_regression_tests
.rs:224:5:
assertion `left == right` failed: SCY=255, screen y=1 should use tile 1 across the scanline
  left: 4289506476
 right: 4283585106
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test game::hardware::ppu_scroll_regression_tests::background_scroll_scy_wraps_from_line_255_to_line_0 ... FAILED
test game::cpu_base_c0_ff_tests::base_c0_ff_immediate_alu_and_memory_io_are_correct ... ok
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU BG: screen=(000,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=2
PPU BG: screen=(001,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=2
PPU BG: screen=(002,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=2
PPU BG: screen=(003,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=2
PPU BG: screen=(004,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=2
PPU BG: screen=(005,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=2
PPU BG: screen=(006,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=2
PPU BG: screen=(007,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=2
PPU BG: screen=(008,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=1
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(009,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
test game::hardware::ppu_tests::background_scanline_reads_tile_map_and_tile_data ... ok
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x80
test game::hardware::ppu_tests::background_scroll_scx_reads_tile_index_1 ... ok
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(020,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(021,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(022,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(023,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(024,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(025,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(026,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(027,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x80
test game::hardware::ppu_tests::background_scroll_scx_reads_color_index_2_from_tile_1 ... ok
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU BG: screen=(000,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=2
PPU BG: screen=(001,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=2
PPU BG: screen=(002,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=2
PPU BG: screen=(003,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=2
PPU BG: screen=(004,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=1
PPU BG: screen=(006,000) source=(006,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,008) window=false map=0x9800 map_index=0x1820 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(009,000) source=(009,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,008) window=false map=0x9800 map_index=0x1821 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,008) window=false map=0x9800 map_index=0x1822 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,008) window=false map=0x9800 map_index=0x1822 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,008) window=false map=0x9800 map_index=0x1822 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,008) window=false map=0x9800 map_index=0x1822 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=2
PPU BG: screen=(005,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=2
PPU BG: screen=(006,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=2
PPU BG: screen=(007,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x01
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
test game::hardware::ppu_tests::background_scroll_scy_selects_shifted_tile_rows_and_wraps_at_256_lines ... ok
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=2
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=1
PPU BG: screen=(008,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=3
PPU BG: screen=(009,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=3
PPU BG: screen=(010,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=3
PPU BG: screen=(011,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=3
PPU BG: screen=(012,000) source=(020,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=3
test game::hardware::ppu_tests::background_scroll_scx_selects_shifted_pixels_and_wraps_at_256_pixels ... ok
test game::hardware::ppu_tests::cgb_background_attributes_decode_palette_bank_flips_and_priority ... ok
PPU BG: screen=(013,000) source=(021,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=3
PPU BG: screen=(014,000) source=(022,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=3
PPU BG: screen=(015,000) source=(023,000) window=false map=0x9800 map_index=0x1802 tile=0x02
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=3
PPU BG: screen=(016,000) source=(024,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(025,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(026,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(027,000) window=false map=0x9800 map_index=0x1803 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
SCY TEST: y=0, scy=255, bg_y=255, tile_y=31, map=0x9800
PPU BG: screen=(000,000) source=(255,255) window=false map=0x9800 map_index=0x1BFF tile=0x1F
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=7 px=7 color_id=3
PPU BG: screen=(001,000) source=(000,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=7 px=0 color_id=0
test game::hardware::ppu_tests::background_map_tile_80_uses_vram_8800_in_unsigned_mode ... ok
test game::hardware::ppu_tests::cgb_palette_autoincrement_writes_consecutive_bytes ... ok
PPU BG: screen=(002,000) source=(001,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=7 px=1 color_id=0
PPU BG: screen=(003,000) source=(002,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=7 px=2 color_id=0
PPU BG: screen=(004,000) source=(003,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=7 px=3 color_id=0
PPU BG: screen=(005,000) source=(004,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=7 px=4 color_id=0
PPU BG: screen=(006,000) source=(005,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=7 px=5 color_id=0
PPU BG: screen=(007,000) source=(006,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=7 px=6 color_id=0
PPU BG: screen=(008,000) source=(007,255) window=false map=0x9800 map_index=0x1BE0 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=7 px=7 color_id=0
PPU BG: screen=(009,000) source=(008,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=7 px=0 color_id=0
PPU BG: screen=(010,000) source=(009,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=7 px=1 color_id=0
PPU BG: screen=(011,000) source=(010,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=7 px=2 color_id=0
PPU BG: screen=(012,000) source=(011,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=7 px=3 color_id=0
PPU BG: screen=(013,000) source=(012,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=7 px=4 color_id=0
PPU BG: screen=(014,000) source=(013,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=7 px=5 color_id=0
PPU BG: screen=(015,000) source=(014,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=7 px=6 color_id=0
PPU BG: screen=(016,000) source=(015,255) window=false map=0x9800 map_index=0x1BE1 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=7 px=7 color_id=0
PPU BG: screen=(017,000) source=(016,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=7 px=0 color_id=0
PPU BG: screen=(018,000) source=(017,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=7 px=1 color_id=0
PPU BG: screen=(019,000) source=(018,255) window=false map=0x9800 map_index=0x1BE2 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=7 px=2 color_id=0
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=1
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x80 palette=0 bank=0 flip_x=false flip_y=false
test game::hardware::ppu_tests::dmg_bgp_palette_maps_all_four_color_indices ... ok
test game::hardware::ppu_tests::background_scroll_wraps_from_bottom_right_edge_to_top_left ... ok
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=1
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=1
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=1
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=1
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=1
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=1
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=1
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=1
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU BG: screen=(000,001) source=(000,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=1 px=0 color_id=0
PPU BG: screen=(001,001) source=(001,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=1 px=1 color_id=0
PPU BG: screen=(002,001) source=(002,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,001) row=1 px=2 color_id=0
PPU BG: screen=(003,001) source=(003,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,001) row=1 px=3 color_id=0
PPU BG: screen=(004,001) source=(004,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=1 px=4 color_id=0
PPU BG: screen=(005,001) source=(005,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=1 px=5 color_id=0
PPU BG: screen=(006,001) source=(006,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=1 px=6 color_id=0
PPU BG: screen=(007,001) source=(007,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=1 px=7 color_id=0
PPU BG: screen=(008,001) source=(008,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=1 px=0 color_id=0
PPU BG: screen=(009,001) source=(009,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=1 px=1 color_id=0
PPU BG: screen=(010,001) source=(010,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=1 px=2 color_id=0
PPU BG: screen=(011,001) source=(011,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
test game::hardware::ppu_tests::cgb_background_priority_tile_is_drawn_over_a_sprite ... ok
test game::hardware::ppu_tests::lcd_disable_and_enable_reset_scanline_state ... ok
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,001) row=1 px=3 color_id=0
PPU BG: screen=(012,001) source=(012,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=1 px=4 color_id=0
PPU BG: screen=(013,001) source=(013,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=1 px=5 color_id=0
PPU BG: screen=(014,001) source=(014,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=1 px=6 color_id=0
PPU BG: screen=(015,001) source=(015,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=1 px=7 color_id=0
PPU BG: screen=(016,001) source=(016,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,001) row=1 px=0 color_id=0
PPU BG: screen=(017,001) source=(017,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=1 px=1 color_id=0
PPU BG: screen=(018,001) source=(018,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=1 px=2 color_id=0
PPU BG: screen=(019,001) source=(019,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=1 px=3 color_id=0
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU BG: screen=(000,002) source=(000,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,002) row=2 px=0 color_id=0
PPU BG: screen=(001,002) source=(001,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,002) row=2 px=1 color_id=0
PPU BG: screen=(002,002) source=(002,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,002) row=2 px=2 color_id=0
PPU BG: screen=(003,002) source=(003,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,002) row=2 px=3 color_id=0
PPU BG: screen=(004,002) source=(004,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,002) row=2 px=4 color_id=0
PPU BG: screen=(005,002) source=(005,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,002) row=2 px=5 color_id=0
PPU BG: screen=(006,002) source=(006,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,002) row=2 px=6 color_id=0
PPU BG: screen=(007,002) source=(007,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,002) row=2 px=7 color_id=0
PPU BG: screen=(008,002) source=(008,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,002) row=2 px=0 color_id=0
PPU BG: screen=(009,002) source=(009,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,002) row=2 px=1 color_id=0
PPU BG: screen=(010,002) source=(010,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,002) row=2 px=2 color_id=0
PPU BG: screen=(011,002) source=(011,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,002) row=2 px=3 color_id=0
PPU BG: screen=(012,002) source=(012,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,002) row=2 px=4 color_id=0
PPU BG: screen=(013,002) source=(013,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,002) row=2 px=5 color_id=0
PPU BG: screen=(014,002) source=(014,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,002) row=2 px=6 color_id=0
PPU BG: screen=(015,002) source=(015,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,002) row=2 px=7 color_id=0
PPU BG: screen=(016,002) source=(016,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,002) row=2 px=0 color_id=0
PPU BG: screen=(017,002) source=(017,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,002) row=2 px=1 color_id=0
PPU BG: screen=(018,002) source=(018,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,002) row=2 px=2 color_id=0
PPU BG: screen=(019,002) source=(019,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,002) row=2 px=3 color_id=0
PPU BG: screen=(000,001) source=(000,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=1 px=0 color_id=1
PPU BG: screen=(001,001) source=(001,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=1 px=1 color_id=1
PPU BG: screen=(002,001) source=(002,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,001) row=1 px=2 color_id=1
PPU BG: screen=(003,001) source=(003,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,001) row=1 px=3 color_id=1
PPU BG: screen=(004,001) source=(004,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=1 px=4 color_id=1
PPU BG: screen=(005,001) source=(005,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=1 px=5 color_id=1
PPU BG: screen=(006,001) source=(006,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=1 px=6 color_id=1
PPU BG: screen=(007,001) source=(007,001) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=1 px=7 color_id=1
PPU BG: screen=(008,001) source=(008,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=1 px=0 color_id=1
PPU BG: screen=(009,001) source=(009,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=1 px=1 color_id=1
PPU BG: screen=(010,001) source=(010,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=1 px=2 color_id=1
PPU BG: screen=(011,001) source=(011,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=3 color_id=0
PPU BG: screen=(001,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=4 color_id=0
PPU BG: screen=(002,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=5 color_id=0
PPU BG: screen=(003,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=6 color_id=0
PPU BG: screen=(004,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU BG: screen=(000,003) source=(000,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=7 color_id=0
PPU BG: screen=(005,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=0 color_id=0
PPU BG: screen=(006,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=1 color_id=0
PPU BG: screen=(007,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=2 color_id=0
PPU BG: screen=(008,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=3 color_id=0
PPU BG: screen=(009,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=4 color_id=0
PPU BG: screen=(010,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=5 color_id=0
PPU BG: screen=(011,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=6 color_id=0
PPU BG: screen=(012,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=7 color_id=0
PPU BG: screen=(013,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=0 color_id=0
PPU BG: screen=(014,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=1 color_id=0
PPU BG: screen=(015,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=2 color_id=0
PPU BG: screen=(016,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=3 color_id=0
PPU BG: screen=(017,000) source=(020,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=4 color_id=0
PPU BG: screen=(018,000) source=(021,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=5 color_id=0
PPU BG: screen=(019,000) source=(022,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=6 color_id=0
PPU PIXEL: screen=(011,001) row=1 px=3 color_id=1
PPU PIXEL: screen=(000,003) row=3 px=0 color_id=0
PPU BG: screen=(001,003) source=(001,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,003) row=3 px=1 color_id=0
PPU BG: screen=(002,003) source=(002,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,003) row=3 px=2 color_id=0
PPU BG: screen=(003,003) source=(003,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,003) row=3 px=3 color_id=0
PPU BG: screen=(004,003) source=(004,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,003) row=3 px=4 color_id=0
PPU BG: screen=(005,003) source=(005,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,003) row=3 px=5 color_id=0
PPU BG: screen=(006,003) source=(006,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,003) row=3 px=6 color_id=0
PPU BG: screen=(007,003) source=(007,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,003) row=3 px=7 color_id=0
PPU BG: screen=(008,003) source=(008,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,003) row=3 px=0 color_id=0
PPU BG: screen=(009,003) source=(009,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,003) row=3 px=1 color_id=0
PPU BG: screen=(010,003) source=(010,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,003) row=3 px=2 color_id=0
PPU BG: screen=(011,003) source=(011,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,003) row=3 px=3 color_id=0
PPU BG: screen=(012,003) source=(012,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,003) row=3 px=4 color_id=0
PPU BG: screen=(013,003) source=(013,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,003) row=3 px=5 color_id=0
PPU BG: screen=(014,003) source=(014,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,003) row=3 px=6 color_id=0
PPU BG: screen=(015,003) source=(015,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,003) row=3 px=7 color_id=0
PPU BG: screen=(016,003) source=(016,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,003) row=3 px=0 color_id=0
PPU BG: screen=(017,003) source=(017,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,003) row=3 px=1 color_id=0
PPU BG: screen=(018,003) source=(018,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,003) row=3 px=2 color_id=0
PPU BG: screen=(019,003) source=(019,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,003) row=3 px=3 color_id=0
PPU BG: screen=(012,001) source=(012,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
test game::cpu_base_80_bf_tests::base_80_bf_alu_operations_produce_correct_results ... ok
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=1 px=4 color_id=1
PPU BG: screen=(013,001) source=(013,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=1 px=5 color_id=1
PPU BG: screen=(014,001) source=(014,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=1 px=6 color_id=1
PPU BG: screen=(015,001) source=(015,001) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=1 px=7 color_id=1
PPU BG: screen=(016,001) source=(016,001) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,001) row=1 px=0 color_id=1
PPU BG: screen=(017,001) source=(017,001) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=1 px=1 color_id=1
PPU BG: screen=(018,001) source=(018,001) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=1 px=2 color_id=1
PPU BG: screen=(019,001) source=(019,001) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=1 px=3 color_id=1
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
test game::hardware::ppu_tests::frame_ready_is_consumed_without_changing_scanline ... ok
PPU BG: screen=(000,002) source=(000,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(000,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,002) row=2 px=0 color_id=1
PPU BG: screen=(001,002) source=(001,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(001,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,002) row=2 px=1 color_id=1
PPU BG: screen=(002,002) source=(002,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(002,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,002) row=2 px=2 color_id=1
PPU BG: screen=(003,002) source=(003,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(003,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,002) row=2 px=3 color_id=1
PPU BG: screen=(004,002) source=(004,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(004,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,002) row=2 px=4 color_id=1
PPU BG: screen=(005,002) source=(005,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(005,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,002) row=2 px=5 color_id=1
PPU BG: screen=(006,002) source=(006,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(006,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,002) row=2 px=6 color_id=1
PPU BG: screen=(007,002) source=(007,002) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(007,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,002) row=2 px=7 color_id=1
PPU BG: screen=(008,002) source=(008,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(008,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,002) row=2 px=0 color_id=1
PPU BG: screen=(009,002) source=(009,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(009,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,002) row=2 px=1 color_id=1
PPU BG: screen=(010,002) source=(010,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(010,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,002) row=2 px=2 color_id=1
PPU BG: screen=(011,002) source=(011,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(011,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,002) row=2 px=3 color_id=1
PPU BG: screen=(012,002) source=(012,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(012,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,002) row=2 px=4 color_id=1
PPU BG: screen=(013,002) source=(013,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(013,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,002) row=2 px=5 color_id=1
PPU BG: screen=(014,002) source=(014,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(014,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,002) row=2 px=6 color_id=1
PPU BG: screen=(015,002) source=(015,002) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(015,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,002) row=2 px=7 color_id=1
PPU BG: screen=(016,002) source=(016,002) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(016,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,002) row=2 px=0 color_id=1
PPU BG: screen=(017,002) source=(017,002) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(017,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,002) row=2 px=1 color_id=1
PPU BG: screen=(018,002) source=(018,002) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(018,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,002) row=2 px=2 color_id=1
PPU BG: screen=(019,002) source=(019,002) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(019,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,002) row=2 px=3 color_id=1
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(000,003) source=(000,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(000,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,003) row=3 px=0 color_id=1
PPU BG: screen=(001,003) source=(001,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(001,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,003) row=3 px=1 color_id=1
PPU BG: screen=(002,003) source=(002,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(002,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,003) row=3 px=2 color_id=1
PPU BG: screen=(003,003) source=(003,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(003,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,003) row=3 px=3 color_id=1
PPU BG: screen=(004,003) source=(004,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(004,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,003) row=3 px=4 color_id=1
PPU BG: screen=(005,003) source=(005,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(005,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU PIXEL: screen=(005,003) row=3 px=5 color_id=1
PPU BG: screen=(006,003) source=(006,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(006,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
test game::hardware::ppu_tests::ppu_scanline_timing_is_exact ... ok
test game::hardware::ppu_tests::mode_3_length_includes_scroll_and_sprite_fetch_penalties ... ok
test game::cpu_timing_tests::conditional_instruction_timing_differs_only_on_taken_path ... ok
test game::hardware::ppu_tests::stat_and_lyc_registers_preserve_expected_bits ... ok
test game::hardware::ppu_tests::tile_data_supports_unsigned_and_signed_addressing ... ok
test game::hardware::ppu_tests::tile_decoder_extracts_four_color_indices ... ok
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(000,000) source=(000,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=2
PPU BG: screen=(001,000) source=(001,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=2
PPU BG: screen=(002,000) source=(002,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=2
PPU BG: screen=(003,000) source=(003,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=2
PPU BG: screen=(004,000) source=(004,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=2
PPU BG: screen=(005,000) source=(005,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=2
PPU BG: screen=(006,000) source=(006,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=2
PPU BG: screen=(007,000) source=(007,000) window=true map=0x9C00 map_index=0x1C00 tile=0x01
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=2
PPU BG: screen=(008,000) source=(008,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=1
PPU BG: screen=(009,000) source=(009,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=1
PPU BG: screen=(010,000) source=(010,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=1
PPU BG: screen=(011,000) source=(011,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=1
PPU BG: screen=(012,000) source=(012,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=1
PPU BG: screen=(013,000) source=(013,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=1
PPU BG: screen=(014,000) source=(014,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=1
PPU BG: screen=(015,000) source=(015,000) window=true map=0x9C00 map_index=0x1C01 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=1
PPU BG: screen=(016,000) source=(016,000) window=true map=0x9C00 map_index=0x1C02 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=1
PPU BG: screen=(017,000) source=(017,000) window=true map=0x9C00 map_index=0x1C02 tile=0x00
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=1
PPU BG: screen=(018,000) source=(018,000) window=true map=0x9C00 map_index=0x1C02 tile=0x00
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=1
PPU BG: screen=(019,000) source=(019,000) window=true map=0x9C00 map_index=0x1C02 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=1
PPU PIXEL: screen=(006,003) row=3 px=6 color_id=1
PPU BG: screen=(007,003) source=(007,003) window=false map=0x9800 map_index=0x1800 tile=0x80
PPU ATTR: screen=(007,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,003) row=3 px=7 color_id=1
PPU BG: screen=(008,003) source=(008,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(008,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,003) row=3 px=0 color_id=1
PPU BG: screen=(009,003) source=(009,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(009,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,003) row=3 px=1 color_id=1
PPU BG: screen=(010,003) source=(010,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(010,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,003) row=3 px=2 color_id=1
PPU BG: screen=(011,003) source=(011,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(011,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,003) row=3 px=3 color_id=1
PPU BG: screen=(012,003) source=(012,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(012,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,003) row=3 px=4 color_id=1
PPU BG: screen=(013,003) source=(013,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(013,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,003) row=3 px=5 color_id=1
PPU BG: screen=(014,003) source=(014,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(014,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,003) row=3 px=6 color_id=1
PPU BG: screen=(015,003) source=(015,003) window=false map=0x9800 map_index=0x1801 tile=0x80
PPU ATTR: screen=(015,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,003) row=3 px=7 color_id=1
PPU BG: screen=(016,003) source=(016,003) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(016,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,003) row=3 px=0 color_id=1
PPU BG: screen=(017,003) source=(017,003) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(017,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,003) row=3 px=1 color_id=1
PPU BG: screen=(018,003) source=(018,003) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(018,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,003) row=3 px=2 color_id=1
PPU BG: screen=(019,003) source=(019,003) window=false map=0x9800 map_index=0x1802 tile=0x80
PPU ATTR: screen=(019,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,003) row=3 px=3 color_id=1
PPU BG: screen=(000,001) source=(000,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,001) row=1 px=0 color_id=0
PPU BG: screen=(001,001) source=(001,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,001) row=1 px=1 color_id=0
PPU BG: screen=(002,001) source=(002,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
test game::hardware::ppu_tests::window_uses_its_own_tile_map_at_wx_minus_seven ... ok
test game::hardware::serial::tests::serial_registers_read_write ... ok
test game::hardware::serial::tests::transfer_generates_interrupt ... ok
test game::hardware::timer::tests::div_increments ... ok
test game::hardware::timer::tests::div_resets_on_write ... ok
test game::hardware::timer::tests::tac_can_disable_timer ... ok
test game::hardware::timer::tests::tima_increments_at_262144hz ... ok
test game::hardware::timer::tests::tima_increments_at_4096hz ... ok
test game::hardware::timer::tests::tima_overflow_reloads_from_tma ... ok
test game::hardware::timer::tests::timer_interrupt_is_taken ... ok
test native::engine::tests::layer_split_preserves_each_original_frame ... ok
test game::hardware::ppu_tests::sprite_rendering_reads_oam_without_changing_bg_scroll ... ok
PPU PIXEL: screen=(002,001) row=1 px=2 color_id=0
PPU BG: screen=(003,001) source=(003,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU BG: screen=(000,000) source=(000,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,000) row=0 px=0 color_id=0
PPU BG: screen=(001,000) source=(001,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,000) row=0 px=1 color_id=0
PPU BG: screen=(002,000) source=(002,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,000) row=0 px=2 color_id=0
PPU BG: screen=(003,000) source=(003,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,000) row=0 px=3 color_id=0
PPU BG: screen=(004,000) source=(004,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,000) row=0 px=4 color_id=0
PPU BG: screen=(005,000) source=(005,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,000) row=0 px=5 color_id=0
PPU BG: screen=(006,000) source=(006,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,000) row=0 px=6 color_id=0
PPU BG: screen=(007,000) source=(007,000) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,000) row=0 px=7 color_id=0
PPU BG: screen=(008,000) source=(008,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,000) row=0 px=0 color_id=0
PPU BG: screen=(009,000) source=(009,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,000) row=0 px=1 color_id=0
PPU BG: screen=(010,000) source=(010,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,000) row=0 px=2 color_id=0
PPU BG: screen=(011,000) source=(011,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,000) row=0 px=3 color_id=0
PPU BG: screen=(012,000) source=(012,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,000) row=0 px=4 color_id=0
PPU BG: screen=(013,000) source=(013,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,000) row=0 px=5 color_id=0
PPU BG: screen=(014,000) source=(014,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,000) row=0 px=6 color_id=0
PPU BG: screen=(015,000) source=(015,000) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,000) row=0 px=7 color_id=0
PPU BG: screen=(016,000) source=(016,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,000) row=0 px=0 color_id=0
PPU BG: screen=(017,000) source=(017,000) window=false map=0x9800 map_index=0x1802 tile=0x00
test game::hardware::ppu_tests::full_frame_background_renders_all_144_visible_scanlines ... ok
PPU ATTR: screen=(017,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,000) row=0 px=1 color_id=0
PPU BG: screen=(018,000) source=(018,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU PIXEL: screen=(003,001) row=1 px=3 color_id=0
PPU BG: screen=(004,001) source=(004,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,001) row=1 px=4 color_id=0
PPU BG: screen=(005,001) source=(005,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,001) row=1 px=5 color_id=0
PPU BG: screen=(006,001) source=(006,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,001) row=1 px=6 color_id=0
PPU BG: screen=(007,001) source=(007,001) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,001) row=1 px=7 color_id=0
PPU BG: screen=(008,001) source=(008,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,001) row=1 px=0 color_id=0
PPU BG: screen=(009,001) source=(009,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,001) row=1 px=1 color_id=0
PPU BG: screen=(010,001) source=(010,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,001) row=1 px=2 color_id=0
PPU BG: screen=(011,001) source=(011,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,001) row=1 px=3 color_id=0
PPU BG: screen=(012,001) source=(012,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,001) row=1 px=4 color_id=0
PPU BG: screen=(013,001) source=(013,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,001) row=1 px=5 color_id=0
PPU BG: screen=(014,001) source=(014,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,001) row=1 px=6 color_id=0
PPU BG: screen=(015,001) source=(015,001) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,001) row=1 px=7 color_id=0
PPU BG: screen=(016,001) source=(016,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,001) row=1 px=0 color_id=0
PPU BG: screen=(017,001) source=(017,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,001) row=1 px=1 color_id=0
PPU BG: screen=(018,001) source=(018,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,001) row=1 px=2 color_id=0
PPU BG: screen=(019,001) source=(019,001) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,001) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,001) row=1 px=3 color_id=0
test game::hardware::ppu_tests::sprite_scanline_uses_only_the_first_ten_oam_entries ... ok
PPU ATTR: screen=(018,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,000) row=0 px=2 color_id=0
PPU BG: screen=(019,000) source=(019,000) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,000) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,000) row=0 px=3 color_id=0
PPU BG: screen=(000,002) source=(000,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,002) row=2 px=0 color_id=0
PPU BG: screen=(001,002) source=(001,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,002) row=2 px=1 color_id=0
PPU BG: screen=(002,002) source=(002,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,002) row=2 px=2 color_id=0
PPU BG: screen=(003,002) source=(003,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,002) row=2 px=3 color_id=0
PPU BG: screen=(004,002) source=(004,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,002) row=2 px=4 color_id=0
PPU BG: screen=(005,002) source=(005,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,002) row=2 px=5 color_id=0
PPU BG: screen=(006,002) source=(006,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,002) row=2 px=6 color_id=0
PPU BG: screen=(007,002) source=(007,002) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,002) row=2 px=7 color_id=0
PPU BG: screen=(008,002) source=(008,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,002) row=2 px=0 color_id=0
PPU BG: screen=(009,002) source=(009,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,002) row=2 px=1 color_id=0
PPU BG: screen=(010,002) source=(010,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,002) row=2 px=2 color_id=0
PPU BG: screen=(011,002) source=(011,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,002) row=2 px=3 color_id=0
PPU BG: screen=(012,002) source=(012,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,002) row=2 px=4 color_id=0
PPU BG: screen=(013,002) source=(013,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,002) row=2 px=5 color_id=0
PPU BG: screen=(014,002) source=(014,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,002) row=2 px=6 color_id=0
PPU BG: screen=(015,002) source=(015,002) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,002) row=2 px=7 color_id=0
PPU BG: screen=(016,002) source=(016,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,002) row=2 px=0 color_id=0
PPU BG: screen=(017,002) source=(017,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,002) row=2 px=1 color_id=0
PPU BG: screen=(018,002) source=(018,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,002) row=2 px=2 color_id=0
PPU BG: screen=(019,002) source=(019,002) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,002) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,002) row=2 px=3 color_id=0
PPU BG: screen=(000,003) source=(000,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(000,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(000,003) row=3 px=0 color_id=0
PPU BG: screen=(001,003) source=(001,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(001,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(001,003) row=3 px=1 color_id=0
PPU BG: screen=(002,003) source=(002,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(002,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(002,003) row=3 px=2 color_id=0
PPU BG: screen=(003,003) source=(003,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(003,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(003,003) row=3 px=3 color_id=0
PPU BG: screen=(004,003) source=(004,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(004,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(004,003) row=3 px=4 color_id=0
PPU BG: screen=(005,003) source=(005,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(005,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(005,003) row=3 px=5 color_id=0
PPU BG: screen=(006,003) source=(006,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(006,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(006,003) row=3 px=6 color_id=0
PPU BG: screen=(007,003) source=(007,003) window=false map=0x9800 map_index=0x1800 tile=0x00
PPU ATTR: screen=(007,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(007,003) row=3 px=7 color_id=0
PPU BG: screen=(008,003) source=(008,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(008,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(008,003) row=3 px=0 color_id=0
PPU BG: screen=(009,003) source=(009,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(009,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(009,003) row=3 px=1 color_id=0
PPU BG: screen=(010,003) source=(010,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(010,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(010,003) row=3 px=2 color_id=0
PPU BG: screen=(011,003) source=(011,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(011,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(011,003) row=3 px=3 color_id=0
PPU BG: screen=(012,003) source=(012,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(012,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(012,003) row=3 px=4 color_id=0
PPU BG: screen=(013,003) source=(013,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(013,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(013,003) row=3 px=5 color_id=0
PPU BG: screen=(014,003) source=(014,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(014,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(014,003) row=3 px=6 color_id=0
PPU BG: screen=(015,003) source=(015,003) window=false map=0x9800 map_index=0x1801 tile=0x00
PPU ATTR: screen=(015,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(015,003) row=3 px=7 color_id=0
PPU BG: screen=(016,003) source=(016,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(016,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(016,003) row=3 px=0 color_id=0
PPU BG: screen=(017,003) source=(017,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(017,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(017,003) row=3 px=1 color_id=0
PPU BG: screen=(018,003) source=(018,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(018,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(018,003) row=3 px=2 color_id=0
PPU BG: screen=(019,003) source=(019,003) window=false map=0x9800 map_index=0x1802 tile=0x00
PPU ATTR: screen=(019,003) attr=0x00 palette=0 bank=0 flip_x=false flip_y=false
PPU PIXEL: screen=(019,003) row=3 px=3 color_id=0
test game::hardware::ppu_tests::ppu_enters_vblank_at_ly_144_and_marks_frame_ready ... ok
test game::cpu_timing_tests::cpu_single_step_cycles_match_machine_cycle_lengths ... ok
test game::cpu_base_c0_ff_tests::base_c0_ff_conditional_jumps_calls_and_returns_cover_both_paths ... ok
test game::cpu_base_c0_ff_tests::base_c0_ff_control_opcodes_have_correct_cycles_and_pc ... ok
test game::cpu_base_80_bf_tests::base_80_bf_all_opcodes_have_correct_cycles_and_pc ... ok
test game::cpu_cb_tests::cb_rotate_shift_group_covers_all_operations_with_carry_and_zero_flags ... ok
test game::cpu_cb_tests::cb_bit_res_set_cover_all_bits_and_all_register_targets ... ok
test game::cpu_cb_tests::cb_all_256_opcodes_have_correct_operation_cycles_and_pc ... ok

failures:

failures:
    game::hardware::ppu_scroll_regression_tests::background_scroll_scy_wraps_from_line_255_to_line_0

test result: FAILED. 150 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.09s

error: test failed, to rerun pass `--lib`
