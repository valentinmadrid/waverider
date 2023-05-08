CREATE TABLE public.accounts (
    account text NOT NULL,
    owner text NOT NULL,
    data bytea,
    executable boolean NOT NULL
);


ALTER TABLE public.accounts OWNER TO postgres;


ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_account_key UNIQUE (account);


ALTER TABLE ONLY public.accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (account);