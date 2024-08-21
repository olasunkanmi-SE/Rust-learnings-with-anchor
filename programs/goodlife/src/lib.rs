use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

pub mod constants;

declare_id!("A9xyxaBsjoBhyg5cRDUdXz1thVbpaHvd3te9jZGEsX2A");

#[program]
pub mod event_management {
    use super::*;

    pub fn create_organizer(ctx: Context<CreateOrganizer>, name: String) -> Result<()> {
        let organizer = &mut ctx.accounts.organizer;
        organizer.name = name;
        organizer.is_active = true;
        organizer.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn create_event(ctx: Context<CreateEvent>, props: EventAttribute) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let organizer = &ctx.accounts.organizer;

        require!(organizer.is_active, ErrorCode::OrganizerNotActive);
        require!(
            Clock::get()?.unix_timestamp < props.date,
            ErrorCode::InvalidDate
        );

        validate_event_attributes(&props)?;

        event.name = props.name;
        event.date = props.date;
        event.venue = props.venue;
        event.total_tickets = props.total_tickets;
        event.available_tickets = props.total_tickets;
        event.base_price = props.base_price;
        event.organizer = organizer.key();
        event.is_active = true;
        Ok(())
    }

    pub fn update_event(ctx: Context<UpdateEvent>, props: EventAttribute) -> Result<()> {
        let event = &mut ctx.accounts.event;

        require!(
            Clock::get()?.unix_timestamp < props.date,
            ErrorCode::InvalidDate
        );
        validate_event_attributes(&props)?;

        event.name = props.name;
        event.date = props.date;
        event.venue = props.venue;
        event.total_tickets = props.total_tickets;
        event.available_tickets = props.total_tickets;
        event.base_price = props.base_price;
        Ok(())
    }
    //complete the functionality
    pub fn close_event(ctx: Context<CloseEvent>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.is_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrganizer<'info> {
    #[account(
        init,
        payer = authority,
        space = constants::ORGANIZER_SPACE,
        seeds = [b"organizer", authority.key().as_ref()],
        bump
    )]
    pub organizer: Account<'info, Organizer>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(
        init,
        payer = authority,
        space = constants::EVENT_SPACE,
        seeds = [b"event", organizer.key().as_ref(), &organizer.event_count.to_le_bytes()],
        bump
    )]
    pub event: Account<'info, Event>,
    #[account(
        mut,
        seeds = [b"organizer", authority.key().as_ref()],
        bump,
        constraint = organizer.authority == authority.key()
    )]
    pub organizer: Account<'info, Organizer>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEvent<'info> {
    #[account(
        mut,
        seeds = [b"event", organizer.key().as_ref(), &event.event_number.to_le_bytes()],
        bump,
        constraint = event.organizer == organizer.key()
    )]
    pub event: Account<'info, Event>,
    #[account(
        seeds = [b"organizer", authority.key().as_ref()],
        bump,
        constraint = organizer.authority == authority.key()
    )]
    pub organizer: Account<'info, Organizer>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseEvent<'info> {
    #[account(
        mut,
        seeds = [b"event", organizer.key().as_ref(), &event.event_number.to_le_bytes()],
        bump,
        constraint = event.organizer == organizer.key(),
        close = authority
    )]
    pub event: Account<'info, Event>,
    #[account(
        seeds = [b"organizer", authority.key().as_ref()],
        bump,
        constraint = organizer.authority == authority.key()
    )]
    pub organizer: Account<'info, Organizer>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct Organizer {
    pub name: String,
    pub is_active: bool,
    pub authority: Pubkey,
    pub event_count: u64,
}

#[account]
pub struct Event {
    pub event_number: u64,
    pub name: String,
    pub date: i64,
    pub venue: String,
    pub total_tickets: u32,
    pub available_tickets: u32,
    pub base_price: u64,
    pub organizer: Pubkey,
    pub is_active: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The organizer is not active")]
    OrganizerNotActive,
    #[msg("Name is required")]
    InvalidName,
    #[msg("Enter a valid future date")]
    InvalidDate,
    #[msg("Venue is required")]
    InvalidVenue,
    #[msg("Total tickets must be greater than zero")]
    InvalidTotalTickets,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct EventAttribute {
    pub name: String,
    pub date: i64,
    pub venue: String,
    pub total_tickets: u32,
    pub base_price: u64,
}

fn validate_event_attributes(props: &EventAttribute) -> Result<()> {
    require!(!props.name.is_empty(), ErrorCode::InvalidName);
    require!(!props.venue.is_empty(), ErrorCode::InvalidVenue);
    require!(props.total_tickets > 0, ErrorCode::InvalidTotalTickets);
    Ok(())
}
