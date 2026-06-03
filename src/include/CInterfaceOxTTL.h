#pragma once

#include <stdio.h>
#include <stdint.h>

#ifndef RDF_TERMTYPE_DEFINED
#define RDF_TERMTYPE_DEFINED
typedef enum {
        URI = 0,
        BNODE = 1,
        TYPEDLITERAL = 2,
        LANGLITERAL = 3
} TERMTYPE;
#endif


typedef struct ttlSerializer TTLSerializer;
typedef struct ttlConfig TTLConfig;
typedef struct trigSerializer TrigSerializer;
typedef struct trigConfig TrigConfig;

#ifndef TRIPLEHANDLER_DEFINED
#define TRIPLEHANDLER_DEFINED
/*
 * Use TERMTYPE for subject_type, object_type and graph_type.
 * If graphid is NULL, the default graph is used.
 */
typedef int8_t TripleHandler(
                const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graphid, uint8_t graph_type,
                void* user);

#endif //TRIPLEHANDLER_DEFINED

#ifdef __cplusplus
//namespace: CInterfaceOxJsonld
extern "C" {
#endif

void free_TTLConfig(TTLConfig*);
TTLConfig* TTLConfig_set_baseiri(TTLConfig*, const char*);

int64_t parse_ttl(const char *input, TripleHandler hook, void* hook_data, TTLConfig* config);

TTLSerializer* TTL_SER_start();
TTLSerializer* TTL_SER_set_base_iri(TTLSerializer*, const char* baseiri);
TTLSerializer* TTL_SER_set_prefix(TTLSerializer*, const char* name, const char* iri);
char* TTL_SER_finish(TTLSerializer*);
int64_t TTL_SER_add(const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graph_id, uint8_t graph_type,
                TTLSerializer* serializer);


void free_TrigConfig(TrigConfig*);
TrigConfig* TrigConfig_set_baseiri(TrigConfig*, const char*);

int64_t parse_trig(const char *input, TripleHandler hook, void* hook_data, TrigConfig* config);

TrigSerializer* Trig_SER_start();
TrigSerializer* Trig_SER_set_base_iri(TrigSerializer*, const char* baseiri);
TrigSerializer* Trig_SER_set_prefix(TrigSerializer*, const char* name, const char* iri);
char* Trig_SER_finish(TrigSerializer*);
int64_t Trig_SER_add(const char* subject, uint8_t subject_type,
                const char* predicate,
                const char* object, const char* object_suffix,
                uint8_t object_type,
                const char* graph_id, uint8_t graph_type,
                TrigSerializer* serializer);

#ifdef __cplusplus
}
#endif
