fn process_swap_event(event: SwapEvent) {
    // Extract pool, tokens, and swap fee
    let pool = event.pool_address;
    let fee = event.fee;
    log::info!("Swap in pool {} with fee {}", pool, fee);
}