// import modules
mod voter_registration;
mod vote_proof;
mod vote_submission;

// import types from imported modules
use voter_registration::{Voter, Party};
use vote_proof::Vote;
use vote_submission::VoteSubmission;

// election simulation
fn main() {
    
    // initialize voting system
    let mut submission = VoteSubmission::new();
    
    println!("\n--- Testing Votes for Each Party ---");

    // create one vote for each party
    for (i, party) in [
        Party::Liberal,
        Party::Conservative, 
        Party::NDP,
        Party::Green
    ].iter().enumerate() {
        
        // create a new voter
        let voter = Voter::new(format!("voter_{}", i));
        println!("\nSubmitting vote for {:?}...", party);
        
        // create and submit the vote
        if let Some(vote) = Vote::new(*party, voter.nullifier(), &submission.bp_gens, &submission.pc_gens) {
            if submission.submit_vote(vote) {
                println!("Vote for {:?} accepted.", party);
            } else {
                println!("Vote for {:?} rejected.", party);
            }
        } else {
            println!("Vote for {:?} failed.", party);
        }

    }

    println!("\n\n--- Testing Double Voting Error Guarding ---");

    // create a new voter that will try to vote twice
    let double_voter = Voter::new("double_voter".to_string());
    let nullifier = double_voter.nullifier();
    
    // duplicate voter testers first vote
    println!("\nFirst Vote: Voting for Liberal...");
    if let Some(vote) = Vote::new(Party::Liberal, nullifier, &submission.bp_gens, &submission.pc_gens) {
        if submission.submit_vote(vote) {
            println!("Vote accepted.");
        } else {
            println!("Vote rejected.");
        }
    }

    // duplicate voter testers second vote
    println!("\nSecond Vote (Duplicate Voter): Voting for Conservative...");
    if let Some(vote) = Vote::new(Party::Conservative, nullifier, &submission.bp_gens, &submission.pc_gens) {
        if submission.submit_vote(vote) {
            println!("Vote accepted.");
        } else {
            println!("Vote rejected, duplicate voter identified.");
        }
    }

    // display election results
    println!("\n\n--- Election Results ---");
    for (party, count) in submission.tally() {
        println!("- {:?}: {}", party, count);
    }

    // display performance results
    submission.display_metrics();
    
}