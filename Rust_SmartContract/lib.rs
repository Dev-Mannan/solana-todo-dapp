use anchor_lang::prelude::*; //importing from library anchor lang "*" mean import all functunalities

pub mod constant;
pub mod error;
pub mod states;

use crate::{constant::*,error::*,states::*}; //using crate to import these files in lib.rs

declare_id!("AHbs8HRdZoboNeGwHsERJcBa71Fi3y5UWe2PxhxZLPfL"); //Build Id

#[program]
pub mod clever_todo{

    //Initialise user
    //add a userprofile in blockchain
    // add values for default data 
    // add todo to the blockchain
    // delete todo 

    use super::*;

    pub fn initialize_user(
        ctx:Context<InitializeUser>


    )-> Result<()>{
        //Write Logic here

        //Initialize user profile with default data
        
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;

        Ok(())

    }

    pub fn add_todo(
        ctx:Context<AddTodo>,
        _content:String
    )->Result<()>{
        
        // Initialise Variables
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill The todo struct with the proper value
        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;

        todo_account.content = _content;
        todo_account.marked = false;

        // Increase Todo idx for PDA 
        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();


        //  Increase Total Todo Count
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())

    }

    pub fn mark_todo(ctx:Context<MarkTodo>,todo_idx:u8)->Result<()>{

        // Change Marked to True -> mark todo as Completed

        let todo_account = &mut ctx.accounts.todo_account;
        require!(!todo_account.marked,TodoError::AlreadyMarked);

        // Mark todo
        todo_account.marked = true;

        Ok(())

    }

    pub fn remove_todo(ctx:Context<RemoveTodo>,todo_idx:u8)->Result<()>{

        // Decrement of Todo Count

        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        // No need to decrease Last todo idx
        // Todo PDA is already closed in context

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub authority:Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG,authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info,UserProfile>>,

    pub system_program:Program<'info,System>,

}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info>{
    #[account(
        mut,
        seeds = [USER_TAG,authority.key().as_ref()],
        bump,
        has_one  = authority,
    )]
    pub user_profile:Box<Account<'info,UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG,authority.key().as_ref(),&[user_profile.last_todo as u8].as_ref()],
        bump,
        payer = authority,
        space = std::mem::size_of::<TodoAccount>()+8,

    )]
    pub todo_account:Box<Account<'info,TodoAccount>>,


    #[account(mut)]
    pub authority:Signer<'info>,

    pub system_program:Program<'info,System>,
}

#[derive(Accounts)]
#[instruction(todo_idx:u8)]
pub struct MarkTodo<'info>{
    #[account(
        mut,
        seeds = [USER_TAG,authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile:Box<Account<'info,UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG,authority.key().as_ref(),&[todo_idx].as_ref()],
        bump,
        has_one = authority,

    )]
    pub todo_account:Box<Account<'info,TodoAccount>>,

    #[account(mut)]
        pub authority:Signer<'info>,

        pub system_program:Program<'info,System>,
}

#[derive(Accounts)]
#[instruction(todo_idx:u8)]
pub struct RemoveTodo<'info>{
    #[account(
        mut,
        seeds = [USER_TAG,authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile:Box<Account<'info,UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [TODO_TAG,authority.key().as_ref(),&[todo_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub todo_account:Box<Account<'info,TodoAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info,System>,

}