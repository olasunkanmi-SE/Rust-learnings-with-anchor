use anchor_lang::prelude::*;
pub mod constants;

declare_id!("A9xyxaBsjoBhyg5cRDUdXz1thVbpaHvd3te9jZGEsX2A");

#[program]
pub mod event_management {
    use super::*;

    pub fn create_organizer(ctx: Context<CreateOrganizer>, name: String) -> Result<()> {
        let organizer = &mut ctx.accounts.organizer;
        organizer.name = name;
        organizer.is_active = true;
        let payer = &ctx.accounts.payer;
        organizer.payer = payer.key();
        return Ok(());
    }

    pub fn create_event(ctx: Context<CreateEvent>, props: EventAttribute) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let organizer = &ctx.accounts.organizer;

        require!(organizer.is_active, ErrorCode::OrganizerNotActive);
        let props_clone = props.clone();
        if let Err(error) = validate_event_attributes(props_clone) {
            return Err(error);
        }
        let payer = &ctx.accounts.payer;
        event.payer = payer.key();
        event.name = props.name;
        event.venue = props.venue;
        event.total_tickets = props.total_tickets;
        event.base_prize = props.base_price;
        event.organizer_id = organizer.key();
        event.date = 0;
        event.available_tickets = props.total_tickets;
        event.is_active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrganizer<'info> {
    #[account(init, payer = payer, space = constants::ORGANIZER_SPACE)]
    pub organizer: Account<'info, Organizer>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Organizer {
    pub organizer_id: Pubkey,
    pub name: String,
    pub is_active: bool,
    pub payer: Pubkey,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = payer, space = constants::EVENT_SPACE)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub organizer: Account<'info, Organizer>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Event {
    pub event_id: Pubkey,
    pub name: String,
    pub date: i64,
    pub venue: String,
    pub total_tickets: u32,
    pub available_tickets: u32,
    pub base_prize: u64,
    pub organizer_id: Pubkey,
    pub is_active: bool,
    pub payer: Pubkey,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The organizer is not active")]
    OrganizerNotActive,
    #[msg("Name is required")]
    InvalidName,
    #[msg("Enter a valid date")]
    InvalidDate,
    #[msg("Venue is required")]
    InvalidVenue,
    #[msg("Name is required")]
    InvalidTotalTickets,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EventAttribute {
    pub name: String,
    pub venue: String,
    pub total_tickets: u32,
    pub base_price: u64,
}

fn validate_event_attributes(props: EventAttribute) -> Result<()> {
    require!(!props.name.is_empty(), ErrorCode::InvalidName);
    require!(!props.venue.is_empty(), ErrorCode::InvalidVenue);
    require!(props.total_tickets > 0, ErrorCode::InvalidTotalTickets);
    Ok(())
}
