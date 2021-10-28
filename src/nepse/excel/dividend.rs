use crate::nepse::response::gain::{DividendListItem, GainListResponse};
use xlsxwriter::*;

fn write_header(sheet: &mut Worksheet) -> Result<(), XlsxError> {
    let headers = vec![
        "Symbol",
        "Company Name",
        "Close Price",
        "Bonus Dividend",
        "Cash Dividend",
        "Total",
        "Average For Total",
        "Average for Bonus Dividend",
        "Average for Cash Dividend",
        "Sector",
        "Number of dividend Record",
        "Book Closure Date",
        "Fiscal Year",
        "Average 3 year Dividend",
        "Dividend Growth Rate",
        "Status",
        "Total Traded Share",
        "Total Trade",
        "Closing Date",
    ];
    for (position, header) in headers.iter().enumerate() {
        sheet.write_string(0, position as u16, header, None)?;
    }
    Ok(())
}

fn format_to_f64(value: Option<String>) -> f64 {
    let empty: String = "0.0".to_string();
    value.unwrap_or(empty).parse().unwrap_or(0.0)
}

fn write_row(
    sheet: &mut Worksheet,
    row: u32,
    dividend: &DividendListItem,
) -> Result<(), XlsxError> {
    let empty: String = "".to_string();
    sheet.write_string(row, 0, dividend.symbol.as_str(), None)?;
    sheet.write_string(
        row,
        1,
        dividend.company_name.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;
    sheet.write_number(row, 2, dividend.close_price.unwrap_or(0.0), None)?;
    sheet.write_number(
        row,
        3,
        format_to_f64(Some(dividend.bonus_dividend.clone())),
        None,
    )?;
    sheet.write_number(
        row,
        4,
        format_to_f64(Some(dividend.cash_dividend.clone())),
        None,
    )?;
    sheet.write_number(row, 5, format_to_f64(Some(dividend.total.clone())), None)?;
    sheet.write_formula(
        row,
        6,
        format!("= AVERAGEIF(A:A, A{}, F:F)", row + 1).as_str(),
        None,
    )?;
    sheet.write_formula(
        row,
        7,
        format!("= AVERAGEIF(A:A, A{}, D:D)", row + 1).as_str(),
        None,
    )?;
    sheet.write_formula(
        row,
        8,
        format!("= AVERAGEIF(A:A, A{}, E:E)", row + 1).as_str(),
        None,
    )?;
    sheet.write_string(
        row,
        9,
        dividend.sector_name.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;
    sheet.write_formula(
        row,
        10,
        format!("= COUNTIF(A:A, A{})", row + 1).as_str(),
        None,
    )?;
    sheet.write_string(
        row,
        11,
        dividend
            .book_closure_date
            .as_ref()
            .unwrap_or(&empty)
            .as_str(),
        None,
    )?;
    sheet.write_string(row, 12, dividend.fiscal_year.as_str(), None)?;
    sheet.write_string(row, 13, dividend.avg_3yr_dividend.as_str(), None)?;
    sheet.write_string(row, 14, dividend.dividend_growth_rate.as_str(), None)?;
    sheet.write_string(row, 15, dividend.status.as_str(), None)?;
    sheet.write_string(
        row,
        16,
        dividend
            .total_traded_shares
            .as_ref()
            .unwrap_or(&empty)
            .as_str(),
        None,
    )?;
    sheet.write_number(row, 17, dividend.total_trades.unwrap_or(0.0), None)?;
    sheet.write_string(
        row,
        18,
        dividend.closing_date.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;

    Ok(())
}

pub fn write_to_excel(
    file_path: String,
    response: Result<GainListResponse<DividendListItem>, reqwest::Error>,
) -> Result<(), XlsxError> {
    if let Ok(list_response) = response {
        let workbook = Workbook::new(file_path.as_str());
        let text_wrap = workbook.add_format().set_text_wrap();
        let mut sheet = workbook.add_worksheet(Some("Dividends"))?;
        write_header(&mut sheet)?;

        for (row, dividend) in list_response.data.iter().enumerate() {
            write_row(&mut sheet, row as u32 + 1, dividend)?;
        }
        sheet.set_column(1, 1, 30.0, None)?;
        sheet.set_column(2, 14, 10.0, Some(&text_wrap))?;
        sheet.autofilter(0, 0, list_response.data.len() as u32 + 2, 19)?;

        println!("Press Ctrl+Shift+F9 in case of libra office to recalculate formulas.");
        workbook.close()?;
    } else {
        println!("Error occured: {:?}", response.unwrap_err());
    }
    Ok(())
}
