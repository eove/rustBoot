#!/bin/sh
DEFMT_LOG=debug cargo +nightly stm32h723 build rustBoot-only && probe-run --chip STM32H723ZGTx ./boards/target/thumbv7em-none-eabihf/release/stm32h723
