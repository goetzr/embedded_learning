#pragma once

#define REG(addr) reinterpret_cast<volatile uint32_t *>((addr))