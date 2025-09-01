/******************************************************************************
 *
 * File: flash_led.c
 * Author: Russ Goetz
 * Date: 31 Aug 2018
 * Description: This program continually flashes an external LED
 *              connected to PB5 every second.
 *
 ******************************************************************************/
 
#include <avr/io.h>
#include <avr/interrupt.h>

void start_timer();

void main()
{
    cli();
    
    // Reset the count
    GPIOR0 = 0;
    
    // Configure PB5 as an output
    DDRB |= _BV(DDB5);
    
    // Turn LED on
    PORTB |= _BV(PORTB5);
    
    start_timer();
    
    sei();
    
    while(1) ;
}

void start_timer()
{
    // Reset the count of times the timer interrupt has fired
    GPIOR1 = 0;
    
    // Reset the timer counter to 0
    TCNT0 = 0;
    
    // Set timer 0 to Normal mode (WGM[2:0] = 0)
    //
    // Set TCCR0A
    // Bits 7:6 (COM0A[1:0]) = 00 (Normal port operation, OC0A disconnected)
    // Bits 5:4 (COM0B[1:0]) = 00 (Normal port operation, OC0B disconnected)
    // Bits 3:2 (Unused)     = 00
    // Bits 1:0 (WGM0[1:0])  = 00 (Normal mode)
    // 0b0000|0000 = 0x00
    TCCR0A = 0x00;
    
    // Set TCCR0B
    // Bit 7 (FOC0A)       = 0
    // Bit 6 (FOC0B)       = 0
    // Bits 5:4 (Unused)   = 00
    // Bit 3 (WGM0[2])     = 0
    // Bits 2:0 (CS0[2:0]) = 101 (clkIO / 1024)
    // 0b0000|0101 = 0x05
    TCCR0B = 0x05;
    
    // Set TC0
    // Bits 7:3 (Unused) = 00000
    // Bit 2 (OCIEB)     = 0
    // Bit 1 (OCIEA)     = 0
    // Bit 0 (TOIE)      = 1
    // 0b0000|0010 = 0x01
    TIMSK0 = 0x01;
}

ISR(TIMER0_OVF_vect)
{
    // 16 MHz CPU clock / 1024 prescaler = 15.625 KHz
    // Want to flash LED at 1 Hz
    // 15.625 KHz / interrupt fires every 256 clock ticks = 61.03515625 Hz
    // Toggle LED state every 61 interrupts: 61.03515625 Hz / 61 = 1.000576 Hz
    if (++GPIOR1 == 61) {
        GPIOR1 = 0;
        if (GPIOR0++ % 2 == 0)
            PORTB &= ~_BV(PORTB5);    // Turn LED off
        else
            PORTB |= _BV(PORTB5);     // Turn LED on
    }
}