#![allow(clippy::useless_vec)]
#![allow(dead_code)]

// DNS Server Example
//
// Demonstrates building a simple DNS server that responds to queries.
// This is a minimal example for educational purposes.

use hickory_proto::op::{Message, MessageType, OpCode, ResponseCode};
use hickory_proto::rr::rdata::A;
use hickory_proto::rr::{RData, Record, RecordType};
use hickory_proto::serialize::binary::{BinDecodable, BinEncodable};
use std::net::{Ipv4Addr, UdpSocket};

fn handle_query(query: &Message) -> Message {
    let mut response = Message::new();

    // Copy query ID and set response flags
    response.set_id(query.id());
    response.set_message_type(MessageType::Response);
    response.set_op_code(OpCode::Query);
    response.set_recursion_desired(query.recursion_desired());
    response.set_recursion_available(false);

    if query.queries().is_empty() {
        response.set_response_code(ResponseCode::FormErr);
        return response;
    }

    let question = &query.queries()[0];
    println!(
        "[Query] {} {} {}",
        question.name(),
        question.query_class(),
        question.query_type()
    );

    // Add the question to response
    response.add_query(question.clone());

    // Handle specific domains (hardcoded for demo)
    let name_str = question.name().to_string();

    match (name_str.as_str(), question.query_type()) {
        ("example.local.", RecordType::A) => {
            // Return 192.168.1.100 for example.local
            let mut record = Record::new();
            record.set_name(question.name().clone());
            record.set_ttl(300);
            record.set_rr_type(RecordType::A);
            record.set_dns_class(question.query_class());
            record.set_data(Some(RData::A(A(Ipv4Addr::new(192, 168, 1, 100)))));

            response.add_answer(record);
            response.set_response_code(ResponseCode::NoError);

            println!("  -> Answered: 192.168.1.100");
        }
        ("test.local.", RecordType::A) => {
            // Return 10.0.0.1 for test.local
            let mut record = Record::new();
            record.set_name(question.name().clone());
            record.set_ttl(300);
            record.set_rr_type(RecordType::A);
            record.set_dns_class(question.query_class());
            record.set_data(Some(RData::A(A(Ipv4Addr::new(10, 0, 0, 1)))));

            response.add_answer(record);
            response.set_response_code(ResponseCode::NoError);

            println!("  -> Answered: 10.0.0.1");
        }
        _ => {
            // Domain not found
            response.set_response_code(ResponseCode::NXDomain);
            println!("  -> Not found");
        }
    }

    response
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DNS Server ===\n");

    let socket = UdpSocket::bind("127.0.0.1:5353")?;
    println!("DNS Server listening on 127.0.0.1:5353");
    println!("\nConfigured domains:");
    println!("  example.local -> 192.168.1.100");
    println!("  test.local    -> 10.0.0.1");
    println!("\nWaiting for queries...\n");

    let mut buf = [0u8; 512];

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, src)) => {
                println!("[Request] From {}", src);

                // Parse DNS query
                match Message::from_bytes(&buf[..size]) {
                    Ok(query) => {
                        // Generate response
                        let response = handle_query(&query);

                        // Serialize and send response
                        match response.to_bytes() {
                            Ok(response_bytes) => {
                                if let Err(e) = socket.send_to(&response_bytes, src) {
                                    eprintln!("  [Error] Failed to send response: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("  [Error] Failed to serialize response: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("  [Error] Failed to parse query: {}", e);
                    }
                }

                println!();
            }
            Err(e) => {
                eprintln!("[Error] Receive failed: {}", e);
            }
        }
    }
}
