/*
 * This file generated automatically from sync.xml by r_client.py.
 * Edit at your peril.
 */

//Make the compiler quiet
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
use std;
use libc::*;
use ffi;
use ffi::xproto;

pub static SYNC_MAJOR_VERSION : c_uint = 3;
pub static SYNC_MINOR_VERSION : c_uint = 1;

pub type alarm = u32;
/**
 * @brief alarm_iterator
 **/
pub struct alarm_iterator {
    pub data : *mut alarm,
    pub rem  : c_int,
    pub index: c_int
}


pub type counter = u32;
/**
 * @brief counter_iterator
 **/
pub struct counter_iterator {
    pub data : *mut counter,
    pub rem  : c_int,
    pub index: c_int
}


pub type fence = u32;
/**
 * @brief fence_iterator
 **/
pub struct fence_iterator {
    pub data : *mut fence,
    pub rem  : c_int,
    pub index: c_int
}


pub struct int64 {
     pub hi :   i32,
     pub lo :   u32
}

/**
 * @brief int64_iterator
 **/
pub struct int64_iterator {
    pub data : *mut int64,
    pub rem  : c_int,
    pub index: c_int
}


pub struct systemcounter {
     pub counter :      counter,
     pub resolution :   int64,
     pub name_len :     u16
}

/**
 * @brief systemcounter_iterator
 **/
pub struct systemcounter_iterator {
    pub data : *mut systemcounter,
    pub rem  : c_int,
    pub index: c_int
}


pub struct trigger {
     pub counter :      counter,
     pub wait_type :    u32,
     pub wait_value :   int64,
     pub test_type :    u32
}

/**
 * @brief trigger_iterator
 **/
pub struct trigger_iterator {
    pub data : *mut trigger,
    pub rem  : c_int,
    pub index: c_int
}


pub struct waitcondition {
     pub trigger :           trigger,
     pub event_threshold :   int64
}

/**
 * @brief waitcondition_iterator
 **/
pub struct waitcondition_iterator {
    pub data : *mut waitcondition,
    pub rem  : c_int,
    pub index: c_int
}



pub struct counter_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16,
     pub bad_counter :     u32,
     pub minor_opcode :    u16,
     pub major_opcode :    u8
}



pub struct alarm_error {
     pub response_type :   u8,
     pub error_code :      u8,
     pub sequence :        u16,
     pub bad_alarm :       u32,
     pub minor_opcode :    u16,
     pub major_opcode :    u8
}


pub struct initialize_cookie {
    sequence : c_uint
}


pub struct initialize_request {
     pub major_opcode :            u8,
     pub minor_opcode :            u8,
     pub length :                  u16,
     pub desired_major_version :   u8,
     pub desired_minor_version :   u8
}


pub struct initialize_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub major_version :   u8,
     pub minor_version :   u8,
     pub pad1 :            [u8;22]
}


pub struct list_system_counters_cookie {
    sequence : c_uint
}


pub struct list_system_counters_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}


pub struct list_system_counters_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub counters_len :    u32,
     pub pad1 :            [u8;20]
}



pub struct create_counter_request {
     pub major_opcode :    u8,
     pub minor_opcode :    u8,
     pub length :          u16,
     pub id :              counter,
     pub initial_value :   int64
}



pub struct destroy_counter_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        counter
}


pub struct query_counter_cookie {
    sequence : c_uint
}


pub struct query_counter_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        counter
}


pub struct query_counter_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub counter_value :   int64
}



pub struct await_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}



pub struct change_counter_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        counter,
     pub amount :         int64
}



pub struct set_counter_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub counter :        counter,
     pub value :          int64
}



pub struct create_alarm_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             alarm,
     pub value_mask :     u32
}



pub struct change_alarm_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             alarm,
     pub value_mask :     u32
}



pub struct destroy_alarm_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub alarm :          alarm
}


pub struct query_alarm_cookie {
    sequence : c_uint
}


pub struct query_alarm_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub alarm :          alarm
}


pub struct query_alarm_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub trigger :         trigger,
     pub delta :           int64,
     pub events :          u8,
     pub state :           u8,
     pub pad1 :            [u8;2]
}



pub struct set_priority_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             u32,
     pub priority :       i32
}


pub struct get_priority_cookie {
    sequence : c_uint
}


pub struct get_priority_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub id :             u32
}


pub struct get_priority_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub priority :        i32
}



pub struct create_fence_request {
     pub major_opcode :          u8,
     pub minor_opcode :          u8,
     pub length :                u16,
     pub drawable :              ffi::xproto::drawable,
     pub fence :                 fence,
     pub initially_triggered :   u8
}



pub struct trigger_fence_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          fence
}



pub struct reset_fence_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          fence
}



pub struct destroy_fence_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          fence
}


pub struct query_fence_cookie {
    sequence : c_uint
}


pub struct query_fence_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16,
     pub fence :          fence
}


pub struct query_fence_reply {
     pub response_type :   u8,
     pub pad0 :            u8,
     pub sequence :        u16,
     pub length :          u32,
     pub triggered :       u8,
     pub pad1 :            [u8;23]
}



pub struct await_fence_request {
     pub major_opcode :   u8,
     pub minor_opcode :   u8,
     pub length :         u16
}



pub struct counter_notify_event {
     pub response_type :   u8,
     pub kind :            u8,
     pub sequence :        u16,
     pub counter :         counter,
     pub wait_value :      int64,
     pub counter_value :   int64,
     pub timestamp :       ffi::xproto::timestamp,
     pub count :           u16,
     pub destroyed :       u8,
     pub pad0 :            u8
}



pub struct alarm_notify_event {
     pub response_type :   u8,
     pub kind :            u8,
     pub sequence :        u16,
     pub alarm :           alarm,
     pub counter_value :   int64,
     pub alarm_value :     int64,
     pub timestamp :       ffi::xproto::timestamp,
     pub state :           u8,
     pub pad0 :            [u8;3]
}

#[link(name="xcb-sync")]
extern "C" {

/**
 * Get the next element of the iterator
 * @param i Pointer to a alarm_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(alarm)
 *
 *
 */
pub fn xcb_sync_alarm_next (i:*mut alarm_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An alarm_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_alarm_end (i:alarm_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a counter_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(counter)
 *
 *
 */
pub fn xcb_sync_counter_next (i:*mut counter_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An counter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_counter_end (i:counter_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a fence_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(fence)
 *
 *
 */
pub fn xcb_sync_fence_next (i:*mut fence_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An fence_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_fence_end (i:fence_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a int64_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(int64)
 *
 *
 */
pub fn xcb_sync_int64_next (i:*mut int64_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An int64_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_int64_end (i:int64_iterator) -> ffi::base::generic_iterator;

pub fn xcb_sync_systemcounter_sizeof (_buffer :  *mut c_void) -> c_int;

pub fn xcb_sync_systemcounter_name (R : *mut systemcounter) -> *mut c_char;


pub fn xcb_sync_systemcounter_name_length (R : *mut systemcounter) -> c_int;


pub fn xcb_sync_systemcounter_name_end (R : *mut systemcounter) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a systemcounter_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(systemcounter)
 *
 *
 */
pub fn xcb_sync_systemcounter_next (i:*mut systemcounter_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An systemcounter_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_systemcounter_end (i:systemcounter_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a trigger_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(trigger)
 *
 *
 */
pub fn xcb_sync_trigger_next (i:*mut trigger_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An trigger_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_trigger_end (i:trigger_iterator) -> ffi::base::generic_iterator;

/**
 * Get the next element of the iterator
 * @param i Pointer to a waitcondition_iterator
 *
 * Get the next element in the iterator. The member rem is
 * decreased by one. The member data points to the next
 * element. The member index is increased by sizeof(waitcondition)
 *
 *
 */
pub fn xcb_sync_waitcondition_next (i:*mut waitcondition_iterator) -> c_void;

/**
 * Return the iterator pointing to the last element
 * @param i An waitcondition_iterator
 * @return  The iterator pointing to the last element
 *
 * Set the current element in the iterator to the last element.
 * The member rem is set to 0. The member data points to the
 * last element.
 */
pub fn xcb_sync_waitcondition_end (i:waitcondition_iterator) -> ffi::base::generic_iterator;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_initialize (c : *mut ffi::base::connection,
                               desired_major_version :  u8,
                               desired_minor_version :  u8) -> initialize_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_initialize_unchecked (c : *mut ffi::base::connection,
                                         desired_major_version :  u8,
                                         desired_minor_version :  u8) -> initialize_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_initialize_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_initialize_reply (c : *mut ffi::base::connection,
                                     cookie : initialize_cookie,
                                     e : *mut *mut ffi::base::generic_error) -> *mut initialize_reply;

pub fn xcb_sync_list_system_counters_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_list_system_counters (c : *mut ffi::base::connection) -> list_system_counters_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_list_system_counters_unchecked (c : *mut ffi::base::connection) -> list_system_counters_cookie;


pub fn xcb_sync_list_system_counters_counters_length (R : *mut list_system_counters_reply) -> c_int;

pub fn xcb_sync_list_system_counters_counters_iterator (R : *mut list_system_counters_reply) -> systemcounter_iterator;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_list_system_counters_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_list_system_counters_reply (c : *mut ffi::base::connection,
                                               cookie : list_system_counters_cookie,
                                               e : *mut *mut ffi::base::generic_error) -> *mut list_system_counters_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_counter_checked (c : *mut ffi::base::connection,
                                           id :  counter,
                                           initial_value :  int64) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_counter (c : *mut ffi::base::connection,
                                   id :  counter,
                                   initial_value :  int64) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_counter_checked (c : *mut ffi::base::connection,
                                            counter :  counter) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_counter (c : *mut ffi::base::connection,
                                    counter :  counter) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_counter (c : *mut ffi::base::connection,
                                  counter :  counter) -> query_counter_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_counter_unchecked (c : *mut ffi::base::connection,
                                            counter :  counter) -> query_counter_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_counter_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_counter_reply (c : *mut ffi::base::connection,
                                        cookie : query_counter_cookie,
                                        e : *mut *mut ffi::base::generic_error) -> *mut query_counter_reply;

pub fn xcb_sync_await_sizeof (_buffer :  *mut c_void,
                       wait_list_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_await_checked (c : *mut ffi::base::connection,
                                  wait_list_len :  u32,
                                  wait_list : *mut waitcondition) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_await (c : *mut ffi::base::connection,
                          wait_list_len :  u32,
                          wait_list : *mut waitcondition) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_change_counter_checked (c : *mut ffi::base::connection,
                                           counter :  counter,
                                           amount :  int64) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_change_counter (c : *mut ffi::base::connection,
                                   counter :  counter,
                                   amount :  int64) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_set_counter_checked (c : *mut ffi::base::connection,
                                        counter :  counter,
                                        value :  int64) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_set_counter (c : *mut ffi::base::connection,
                                counter :  counter,
                                value :  int64) -> ffi::base::void_cookie;

pub fn xcb_sync_create_alarm_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_alarm_checked (c : *mut ffi::base::connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_alarm (c : *mut ffi::base::connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::void_cookie;

pub fn xcb_sync_change_alarm_sizeof (_buffer :  *mut c_void) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_change_alarm_checked (c : *mut ffi::base::connection,
                                         id :  alarm,
                                         value_mask :  u32,
                                         value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_change_alarm (c : *mut ffi::base::connection,
                                 id :  alarm,
                                 value_mask :  u32,
                                 value_list : *mut u32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_alarm_checked (c : *mut ffi::base::connection,
                                          alarm :  alarm) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_alarm (c : *mut ffi::base::connection,
                                  alarm :  alarm) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_alarm (c : *mut ffi::base::connection,
                                alarm :  alarm) -> query_alarm_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_alarm_unchecked (c : *mut ffi::base::connection,
                                          alarm :  alarm) -> query_alarm_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_alarm_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_alarm_reply (c : *mut ffi::base::connection,
                                      cookie : query_alarm_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut query_alarm_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_set_priority_checked (c : *mut ffi::base::connection,
                                         id :  u32,
                                         priority :  i32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_set_priority (c : *mut ffi::base::connection,
                                 id :  u32,
                                 priority :  i32) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_get_priority (c : *mut ffi::base::connection,
                                 id :  u32) -> get_priority_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_get_priority_unchecked (c : *mut ffi::base::connection,
                                           id :  u32) -> get_priority_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_get_priority_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_get_priority_reply (c : *mut ffi::base::connection,
                                       cookie : get_priority_cookie,
                                       e : *mut *mut ffi::base::generic_error) -> *mut get_priority_reply;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_create_fence_checked (c : *mut ffi::base::connection,
                                         drawable :  ffi::xproto::drawable,
                                         fence :  fence,
                                         initially_triggered :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_create_fence (c : *mut ffi::base::connection,
                                 drawable :  ffi::xproto::drawable,
                                 fence :  fence,
                                 initially_triggered :  u8) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_trigger_fence_checked (c : *mut ffi::base::connection,
                                          fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_trigger_fence (c : *mut ffi::base::connection,
                                  fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_reset_fence_checked (c : *mut ffi::base::connection,
                                        fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_reset_fence (c : *mut ffi::base::connection,
                                fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_destroy_fence_checked (c : *mut ffi::base::connection,
                                          fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_destroy_fence (c : *mut ffi::base::connection,
                                  fence :  fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_query_fence (c : *mut ffi::base::connection,
                                fence :  fence) -> query_fence_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will cause
 * a reply to be generated. Any returned error will be
 * placed in the event queue.
 */
pub fn xcb_sync_query_fence_unchecked (c : *mut ffi::base::connection,
                                          fence :  fence) -> query_fence_cookie;

/**
 * Return the reply
 * @param c      The connection
 * @param cookie The cookie
 * @param e      The generic_error supplied
 *
 * Returns the reply of the request asked by
 * 
 * The parameter @p e supplied to this function must be NULL if
 * xcb_sync_query_fence_unchecked(). is used.
 * Otherwise, it stores the error if any.
 *
 * The returned value must be freed by the caller using free().
 */
pub fn xcb_sync_query_fence_reply (c : *mut ffi::base::connection,
                                      cookie : query_fence_cookie,
                                      e : *mut *mut ffi::base::generic_error) -> *mut query_fence_reply;

pub fn xcb_sync_await_fence_sizeof (_buffer :  *mut c_void,
                             fence_list_len :  u32) -> c_int;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 * This form can be used only if the request will not cause
 * a reply to be generated. Any returned error will be
 * saved for handling by xcb_request_check().
 */
pub fn xcb_sync_await_fence_checked (c : *mut ffi::base::connection,
                                        fence_list_len :  u32,
                                        fence_list : *mut fence) -> ffi::base::void_cookie;

/**
 *
 * @param c The connection
 * @return A cookie
 *
 * Delivers a request to the X server.
 * 
 */
pub fn xcb_sync_await_fence (c : *mut ffi::base::connection,
                                fence_list_len :  u32,
                                fence_list : *mut fence) -> ffi::base::void_cookie;
}

