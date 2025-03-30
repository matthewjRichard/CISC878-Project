// import modules
mod circuit;
mod voter_registration;
mod vote_proof;
mod vote_submission;

// import types from imported modules
use voter_registration::{Voter, Party};
use vote_proof::SnarkVote;
use vote_submission::VoteSubmission;

// election simulation
fn main() {
    
    // create voting system
    let mut submission = VoteSubmission::new();
    
    println!("\n--- Testing Votes for Each Party ---");
    
    // array of possible votes
    let parties = [
        Party::Liberal,
        Party::Conservative, 
        Party::NDP,
        Party::Green
    ];

    // create one vote for each party
    for (i, party) in parties.iter().enumerate() {
        
        // create a new voter
        let voter = Voter::new(format!("voter_{}", i));
        
        println!("\nSubmitting vote for {:?}...", party);
        
        // create a zk-SNARK proof
        let vote = SnarkVote::new(*party, voter.nullifier(), &submission.pk);
        
        // submit the vote
        if submission.submit_vote(vote) {
            println!("Vote for {:?} accepted.", party);
        } else {
            println!("Vote for {:?} rejected.", party);
        }
    }

    println!("\n\n--- Testing Double Voting Error Guarding ---");
    
    // create a new voter that will try to vote twice
    let double_voter = Voter::new("double_voter".to_string());
    let nullifier = double_voter.nullifier();
    
    // duplicate voter testers first vote
    println!("\nFirst Vote: Voting for Liberal...");
    let vote1 = SnarkVote::new(Party::Liberal, nullifier, &submission.pk);
    if submission.submit_vote(vote1) {
        println!("Vote accepted.");
    } else {
        println!("Vote rejected.");
    }

    // duplicate voter testers second vote
    println!("\nSecond Vote (Duplicate Voter): Voting for Conservative...");
    let vote2 = SnarkVote::new(Party::Conservative, nullifier, &submission.pk);
    if submission.submit_vote(vote2) {
        println!("Vote accepted.");
    } else {
        println!("Vote rejected, duplicate voter identified.");
    }

    println!("\n\n--- Election Results ---");
    
    // count votes for each party
    let results = submission.tally();
    
    // display election results
    for (party_id, count) in results {
        let party_name = Party::from_u64(party_id);
        println!("- {:?}: {} votes", party_name, count);
    }

    // display performance results
    submission.display_metrics();
    
}