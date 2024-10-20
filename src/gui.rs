GUI
    let gui_options = eframe::NativeOptions::default();

    let mut budget_create_window = false;

    run_simple_native("Budget Manager", gui_options, move |_ctx, _frame| {
        egui::CentralPanel::default().show(_ctx, |ui| {
            ui.label("Budget Manager");
            if ui.button("Add budget").clicked() {
                budget_create_window = true;
            }
            ui.label("Available budgets");
            // for budget in &budgets.budgets {
            //     ui.horizontal(|ui| {
            //         let name = budget.name.clone();
            //         // ui.button(name);
            //         ui.label(name);
            //     });
            // }
            egui::Grid::new("Budget").show(ui, |ui| {
                let budget = &budgets.budgets.get_mut(current_budget.unwrap()).unwrap();
                let ballance = budget.get_ballance().to_string();
                ui.label(ballance);
            })
        });

        if budget_create_window {
            egui::Window::new("Create Budget Window").show(_ctx, |ui| {
                ui.label("Create Budget Window");

                if ui.button("Close").clicked() {
                    budget_create_window = false;
                }
            });
        }
    })