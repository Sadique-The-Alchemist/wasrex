defmodule Wasrex.Application do
  # See https://hexdocs.pm/elixir/Application.html
  # for more information on OTP Applications
  @moduledoc false

  use Application

  @impl true
  def start(_type, _args) do
    children = [
      WasrexWeb.Telemetry,
      {DNSCluster, query: Application.get_env(:wasrex, :dns_cluster_query) || :ignore},
      {Phoenix.PubSub, name: Wasrex.PubSub},
      # Start the Finch HTTP client for sending emails
      {Finch, name: Wasrex.Finch},
      # Start a worker by calling: Wasrex.Worker.start_link(arg)
      # {Wasrex.Worker, arg},
      # Start to serve requests, typically the last entry
      WasrexWeb.Endpoint
    ]

    # See https://hexdocs.pm/elixir/Supervisor.html
    # for other strategies and supported options
    opts = [strategy: :one_for_one, name: Wasrex.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    WasrexWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
