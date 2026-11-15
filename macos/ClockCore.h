#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct KCClock KCClock;
typedef struct {
    uint64_t generation, revision;
    int32_t hour, minute, second, visitor, door, sprite, scheduled;
    float swing, elapsed, depth, lateral;
} KCFrame;

// Main thread only. Copy borrowed buffers before the next corresponding call.
KCClock *kc_create(const uint8_t *bytes, size_t length);
void kc_destroy(KCClock *clock);
enum { KCSettings, KCSaved, KCCast };
const uint8_t *kc_json(KCClock *clock, int32_t kind, size_t *length);
bool kc_settings(KCClock *clock, const uint8_t *bytes, size_t length);
void kc_tick(KCClock *clock, KCFrame *frame);
void kc_visit(KCClock *clock, int32_t id, bool half, bool scheduled);
void kc_stop(KCClock *clock);
void kc_resume(KCClock *clock);
const uint8_t *kc_audio(KCClock *clock, size_t *length);
void kc_started(KCClock *clock);
