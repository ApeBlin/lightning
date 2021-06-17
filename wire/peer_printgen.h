/* This file was generated by generate-wire.py */
/* Do not modify this file! Modify the .csv file it was generated from. */
/* Template located at tools/gen/print_header_template */
#ifndef LIGHTNING_WIRE_PEER_PRINTGEN_H
#define LIGHTNING_WIRE_PEER_PRINTGEN_H
#include <ccan/tal/tal.h>
#include <devtools/print_wire.h>

void printpeer_wire_message(const u8 *msg);

void printpeer_wire_tlv_message(const char *tlv_name, const u8 *msg);

void printwire_init(const char *fieldname, const u8 *cursor);

void printwire_error(const char *fieldname, const u8 *cursor);

void printwire_warning(const char *fieldname, const u8 *cursor);

void printwire_ping(const char *fieldname, const u8 *cursor);

void printwire_pong(const char *fieldname, const u8 *cursor);

void printwire_tx_add_input(const char *fieldname, const u8 *cursor);

void printwire_tx_add_output(const char *fieldname, const u8 *cursor);

void printwire_tx_remove_input(const char *fieldname, const u8 *cursor);

void printwire_tx_remove_output(const char *fieldname, const u8 *cursor);

void printwire_tx_complete(const char *fieldname, const u8 *cursor);

void printwire_tx_signatures(const char *fieldname, const u8 *cursor);

void printwire_open_channel(const char *fieldname, const u8 *cursor);

void printwire_accept_channel(const char *fieldname, const u8 *cursor);

void printwire_funding_created(const char *fieldname, const u8 *cursor);

void printwire_funding_signed(const char *fieldname, const u8 *cursor);

void printwire_funding_locked(const char *fieldname, const u8 *cursor);

void printwire_open_channel2(const char *fieldname, const u8 *cursor);

void printwire_accept_channel2(const char *fieldname, const u8 *cursor);

void printwire_init_rbf(const char *fieldname, const u8 *cursor);

void printwire_ack_rbf(const char *fieldname, const u8 *cursor);

void printwire_shutdown(const char *fieldname, const u8 *cursor);

void printwire_closing_signed(const char *fieldname, const u8 *cursor);

void printwire_update_add_htlc(const char *fieldname, const u8 *cursor);

void printwire_update_fulfill_htlc(const char *fieldname, const u8 *cursor);

void printwire_update_fail_htlc(const char *fieldname, const u8 *cursor);

void printwire_update_fail_malformed_htlc(const char *fieldname, const u8 *cursor);

void printwire_commitment_signed(const char *fieldname, const u8 *cursor);

void printwire_revoke_and_ack(const char *fieldname, const u8 *cursor);

void printwire_update_fee(const char *fieldname, const u8 *cursor);

void printwire_channel_reestablish(const char *fieldname, const u8 *cursor);

void printwire_announcement_signatures(const char *fieldname, const u8 *cursor);

void printwire_channel_announcement(const char *fieldname, const u8 *cursor);

void printwire_node_announcement(const char *fieldname, const u8 *cursor);

void printwire_channel_update(const char *fieldname, const u8 *cursor);

void printwire_query_short_channel_ids(const char *fieldname, const u8 *cursor);

void printwire_reply_short_channel_ids_end(const char *fieldname, const u8 *cursor);

void printwire_query_channel_range(const char *fieldname, const u8 *cursor);

void printwire_reply_channel_range(const char *fieldname, const u8 *cursor);

void printwire_gossip_timestamp_filter(const char *fieldname, const u8 *cursor);

void printwire_onion_message(const char *fieldname, const u8 *cursor);


void printwire_witness_element(const char *fieldname, const u8 **cursor, size_t *plen);
void printwire_channel_update_checksums(const char *fieldname, const u8 **cursor, size_t *plen);
void printwire_channel_update_timestamps(const char *fieldname, const u8 **cursor, size_t *plen);
void printwire_witness_stack(const char *fieldname, const u8 **cursor, size_t *plen);
#endif /* LIGHTNING_WIRE_PEER_PRINTGEN_H */
// SHA256STAMP:d4f6f16581d26f95c512a5a98e962abe529ff37a70a7563bd41f25ac802bdb63
